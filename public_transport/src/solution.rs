use core::{cmp::Ordering, panic};
use std::{
    cell::RefCell,
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

        self.buses.push(RefCell::new(Box::new(Bus {
            route: destinations.iter().cloned().cloned().collect(),
            current_city: None,
            next_city: Some(first.clone()),
            next_city_counter: 0,
            distance_remaining: 1,
            is_finished: false,
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
        let mut current_events = Vec::new();

        for i_step in 0..time_steps {
            //println!("Step {}", i_step);
            for bus in &mut self.buses {
                let mut bus_mut = bus.borrow_mut();

                if bus_mut.is_finished {
                    continue;
                }

                // update distance remaining
                bus_mut.distance_remaining -= 1;
                //println!("Distance for bus: {}", bus_mut.distance_remaining);

                let bus_stop_happening = bus_mut.distance_remaining == 0;
                let mut curr_event = Box::new(Event {
                    city: bus_mut.next_city.clone().unwrap(),
                    off: 0,
                    on: 0,
                });

                // DONE: check for people to get off
                {
                    let mut temp_people = Vec::new();
                    while let Some(p) = bus_mut.people_onboard.pop() {
                        if bus_mut.distance_remaining > 0 {
                            temp_people.push(p);
                            continue;
                        }

                        let cond = p.to == bus_mut.next_city.clone().unwrap();
                        //println!("Step {}: People get off? {}", i_step, cond);
                        // people get off
                        if cond
                        /* && bus_mut.distance_remaining == 0 */
                        {
                            //let city = bus_mut.next_city.clone().unwrap();

                            // let evnt = Event {
                            //     city: city.clone(),
                            //     on: 0,
                            //     off: p.amount,
                            //     step_number: i_step
                            // };
                            // current_events.push(evnt);
                            curr_event.as_mut().off += p.amount;
                        } else {
                            temp_people.push(p);
                        }
                    }
                    bus_mut.people_onboard = temp_people;
                }

                // DONE: check for people to get on
                {
                    let mut temp_people = Vec::new();
                    for people in self.people.iter() {
                        let city = bus_mut.next_city.clone().unwrap();
                        let next_cities: Vec<_> = (bus_mut.next_city_counter..bus_mut.route.len())
                            .into_iter()
                            .map(|city_route_index| bus_mut.route.get(city_route_index).unwrap())
                            .collect();
                        if bus_mut.distance_remaining == 0
                            && city == people.from
                            && next_cities.contains(&&people.to)
                        {
                            bus_mut.people_onboard.push(people.clone());
                            curr_event.as_mut().on += people.amount;
                            //break;
                        } else {
                            temp_people.push(people.clone());
                        }
                    }
                    //self.people.clear();
                    self.people = temp_people;
                }
                // DONE: update to next destination if neccessary
                {
                    if bus_mut.distance_remaining == 0
                        && &bus_mut.next_city.clone().unwrap() != bus_mut.route.last().unwrap()
                    {
                        bus_mut.current_city = bus_mut.next_city.clone();
                        bus_mut.next_city_counter += 1;

                        // bus_mut.next_city will be None if bus should be deleted
                        bus_mut.next_city = bus_mut
                            .route
                            .iter()
                            .cloned()
                            .nth(bus_mut.next_city_counter as usize);
                        if let Some(next_city) = bus_mut.next_city.clone() {
                            bus_mut.distance_remaining = *self
                                .edges
                                .get(&order_pair(&bus_mut.current_city.clone().unwrap(), &next_city))
                                .unwrap();
                        } else {
                            // prepare for deletion
                            bus_mut.distance_remaining = -1;
                        }
                    }
                }

                if bus_stop_happening {
                    current_events.push(*curr_event);
                }

                // DONE: remove buses that ended
                if bus_mut.distance_remaining == -1 {
                    bus_mut.is_finished = true;
                }
            }

            let evnts: Vec<_> = current_events.iter().cloned().collect();
            for evnt in evnts {
                events.push(evnt);
            }
            current_events.clear();

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

impl Vertex {
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Edge {
    v1: Rc<Box<Vertex>>,
    v2: Rc<Box<Vertex>>,
    value: i32,
}

#[derive(Debug, Clone)]
pub struct Bus {
    route: Vec<Rc<Box<Vertex>>>,
    current_city: Option<Rc<Box<Vertex>>>,
    next_city: Option<Rc<Box<Vertex>>>,
    next_city_counter: usize,
    distance_remaining: i32,
    is_finished: bool,
    people_onboard: Vec<Rc<Box<People>>>,
}

#[derive(Debug)]
pub struct People {
    from: Rc<Box<Vertex>>,
    to: Rc<Box<Vertex>>,
    amount: i32,
}

#[derive(Clone)]
pub struct Event {
    city: Rc<Box<Vertex>>,
    on: i32,
    off: i32,
}

impl Event {
    pub fn city(&self) -> Rc<Box<Vertex>> {
        self.city.clone()
    }
    pub fn got_on(&self) -> i32 {
        self.on
    }
    pub fn got_off(&self) -> i32 {
        self.off
    }
}

fn order_pair(v1: &Rc<Box<Vertex>>, v2: &Rc<Box<Vertex>>) -> (Rc<Box<Vertex>>, Rc<Box<Vertex>>) {
    let c1 = if v1 < v2 { v1 } else { v2 };
    let c2 = if v1 < v2 { v2 } else { v1 };

    (c1.clone(), c2.clone())
}
