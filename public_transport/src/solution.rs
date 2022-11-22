use core::{cmp::Ordering, panic, time};
use std::{
    cell::{Ref, RefCell},
    collections::{HashMap, HashSet},
    rc::Rc,
};

#[derive(Debug)]
pub struct Simulation {
    vertices: HashSet<Rc<Box<Vertex>>>,
    edges: HashMap<(Rc<Box<Vertex>>, Rc<Box<Vertex>>), i32>,
    buses: Vec<RefCell<Box<Bus>>>,
    people: Vec<Rc<Box<People>>>,
    current_time: u32,
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            vertices: HashSet::new(),
            edges: HashMap::new(),
            buses: vec![],
            people: Vec::new(),
            current_time: 0,
        }
    }
    pub fn new_city(&mut self, city_name: &str) -> Rc<Box<Vertex>> {
        let c = Rc::new(Box::new(Vertex {
            name: city_name.to_string(),
        }));

        let res = c.clone();
        self.vertices.insert(c);

        res
    }
    pub fn new_road(
        &mut self,
        city1: &Rc<Box<Vertex>>,
        city2: &Rc<Box<Vertex>>,
        distance: i32,
    ) -> Rc<Box<Edge>> {
        // check if the cities are in Simulation
        if !self.vertices.contains(city1) || !self.vertices.contains(city2) {
            panic!("One of cities not in current simulation.");
        }

        let edge = Rc::new(Box::new(Edge {
            v1: city1.clone(),
            v2: city2.clone(),
            value: distance,
        }));

        let (c1, c2) = order_pair(&city1, &city2);

        self.edges.insert((c1.clone(), c2.clone()), distance);

        edge
    }
    pub fn new_bus(&mut self, destinations: &[&Rc<Box<Vertex>>]) {
        // check if stop are connected
        if destinations.len() < 2 {
            panic!("Too few destinations for bus.");
        }
        for high in 1..destinations.len() {
            let first = destinations.get(high).unwrap();
            let second = destinations.get(high - 1).unwrap();

            let (c1, c2) = order_pair(*first, *second);

            if !self.edges.contains_key(&(c1, c2)) {
                panic!("No edge between cities.");
            }
        }

        let first = destinations.get(0).unwrap().clone();
        let second = destinations.get(1).unwrap().clone();

        self.buses.push(RefCell::new(Box::new(Bus {
            route: destinations.iter().cloned().cloned().collect(),
            current_city: first.clone(),
            next_city: second.clone(),
            next_city_counter: 1,
            distance_remaining: *self.edges.get(&order_pair(first, second)).unwrap(),
            people_onboard: Vec::new(),
        })));
    }
    pub fn add_people(&mut self, from: &Rc<Box<Vertex>>, to: &Rc<Box<Vertex>>, amount: i32) {
        self.people.push(Rc::new(Box::new(People {
            from: from.clone(),
            to: to.clone(),
            amount: amount,
        })));
    }
    pub fn execute(&mut self, time_steps: i32) -> Vec<Event> {
        let mut events = Vec::new();

        // board people in begining cities
        for people in &self.people {
            for bus in &mut self.buses {
                let mut bus_mut = bus.borrow_mut();
                if bus_mut.current_city == people.from {
                    bus_mut.people_onboard.push(people.clone());
                    break;
                }
            }
        }

        for _ in 0..time_steps {
            for bus in &mut self.buses {
                let mut bus_mut = bus.borrow_mut();
                bus_mut.distance_remaining -= 1;

                // TODO: check for people to get off
                
            }


            // TODO: check for people to get on

            // DONE: if bus is empty and in final destination -> erase
            let mut buses = Vec::new();
            while let Some(bus) = self.buses.pop() {
                buses.push(bus);
            }
            for bus in buses.into_iter().rev().filter(|bus| {
                let bus_borrowed = bus.borrow();

                !(bus_borrowed.distance_remaining == 0
                    && bus_borrowed.people_onboard.is_empty()
                    && &bus_borrowed.next_city == bus_borrowed.route.last().unwrap())
            }) {
                self.buses.push(bus);
            }

            // DONE: update to next destination if neccessary
            for bus in &self.buses {
                let mut bus_mut = bus.borrow_mut();

                if bus_mut.distance_remaining == 0
                    && &bus_mut.next_city != bus_mut.route.last().unwrap()
                {
                    bus_mut.current_city = bus_mut.next_city.clone();
                    bus_mut.next_city_counter += 1;
                    bus_mut.next_city = bus_mut
                        .route
                        .get(bus_mut.next_city_counter as usize)
                        .unwrap()
                        .clone();
                    bus_mut.distance_remaining = *self
                        .edges
                        .get(&order_pair(&bus_mut.current_city, &bus_mut.next_city))
                        .unwrap();
                }
            }

            self.current_time += 1;
        }

        events
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Vertex {
    name: String,
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Edge {
    v1: Rc<Box<Vertex>>,
    v2: Rc<Box<Vertex>>,
    value: i32,
}

#[derive(Debug)]
pub struct Bus {
    route: Vec<Rc<Box<Vertex>>>,
    current_city: Rc<Box<Vertex>>,
    next_city: Rc<Box<Vertex>>,
    next_city_counter: usize,
    distance_remaining: i32,
    people_onboard: Vec<Rc<Box<People>>>,
}

#[derive(Debug)]
pub struct People {
    from: Rc<Box<Vertex>>,
    to: Rc<Box<Vertex>>,
    amount: i32,
}

pub struct Event {
    city: Rc<Box<Vertex>>,
    on: u32,
    off: u32,
}

fn order_pair(v1: &Rc<Box<Vertex>>, v2: &Rc<Box<Vertex>>) -> (Rc<Box<Vertex>>, Rc<Box<Vertex>>) {
    let c1 = if v1 < v2 { v1 } else { v2 };
    let c2 = if v1 < v2 { v2 } else { v1 };

    (c1.clone(), c2.clone())
}
