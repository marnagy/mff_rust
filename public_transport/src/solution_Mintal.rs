type RoadHandle = usize;
type CityHandle = usize;
type BusHandle  = usize; 
type CityName   = String;

pub struct CityNameStruct {
    city: CityName,
}

impl CityNameStruct {
    pub fn name(&self) -> String {
        return self.city.clone();
    }
}

#[derive(Debug, Clone)]
pub struct BusStopEvent {
    city: CityName,
    got_on: usize,
    got_off: usize,
}

impl BusStopEvent {

    pub fn city(&self) -> CityNameStruct {
        return CityNameStruct{ city: self.city.clone() };
    }

    pub fn got_off(&self) -> usize {
        return self.got_off;
    }

    pub fn got_on(&self) -> usize {
        return self.got_on;
    }
}

pub struct Road {
    city_0: CityHandle,
    city_1  : CityHandle,
    duration : usize,
}

pub struct PeopleTransit {
    city_from: CityHandle,
    city_to  : CityHandle,
    amount   : usize,
    peoples_bus : Option<BusHandle>,
    got_off: bool,
} 

pub struct Bus {
    stops: Vec<CityHandle>,
    ttf_roads: Vec<usize>,
    cur_road_idx: i32,
    remaining_ttf_road: usize,
    got_done: bool
}

impl Bus {
    pub fn new(stops: &[&CityHandle], ttf_roads: Vec<usize>) -> Bus {
        let mut tmp_stops: Vec<CityHandle> = Vec::new();
       
        for &s in stops.iter() {
            tmp_stops.push(*s);
        }

        return Bus{
            stops: tmp_stops,
            ttf_roads: ttf_roads,
            cur_road_idx: -1,
            remaining_ttf_road: 0, //if zero bus is in stop stops[cur_road_idx + 1]
            got_done: false,
        };
    }

    pub fn is_at_stop(&self) -> bool {
        return self.remaining_ttf_road == 0;
    }

    pub fn get_curr_stop(&self) -> CityHandle {
        if self.remaining_ttf_road == 0 {
            return self.stops[(self.cur_road_idx + 1) as usize];
        }

        panic!("Bus not at any stop");
    }

    //returns bool --- if true bus still moved, else it has finished it's jurney
    pub fn step(&mut self) -> bool {

        //If i am in my last road and i have finished it i wont move further
        if (self.cur_road_idx == (self.ttf_roads.len() as i32) - 1) && (self.remaining_ttf_road == 0) {
            return false;
        }
        else {
            if self.remaining_ttf_road == 0 {
                self.cur_road_idx += 1;
                self.remaining_ttf_road = self.ttf_roads[self.cur_road_idx as usize];
            }

            self.remaining_ttf_road -= 1;
            return true;
        }
    }

    //Can be called only at stop
    pub fn cities_yet_to_visit(&self) -> Vec<CityHandle> {

        if self.is_at_stop() {
            let mut ret: Vec<CityHandle> = Vec::new();
            for i in (self.cur_road_idx + 2) as usize..self.stops.len() {
                ret.push(self.stops[i]);
            }

            return ret;
        }

        panic!("Can be queried only at stop");
    }
}

pub struct Simulation {
    cities: Vec<CityName>,
    roads : Vec<Road>,
    people: Vec<PeopleTransit>,
    buses: Vec<Bus>,
}

impl Simulation {
    pub fn new() -> Simulation {
        return Simulation{
            cities: Vec::new(),
            roads : Vec::new(),
            people: Vec::new(),
            buses : Vec::new()
        };
    }

    pub fn new_city(&mut self, city_name: &str) -> CityHandle {
        let city_name: CityName = city_name.to_string();
        self.cities.push(city_name);

        return self.cities.len() - 1;
    }

    pub fn new_road(&mut self, city_0: &CityHandle, city_1: &CityHandle, duration: usize) -> RoadHandle {
        let road: Road = Road{ 
            city_0: city_0.clone(), 
            city_1: city_1.clone(), 
            duration: duration
        };
        self.roads.push(road);

        return self.roads.len() - 1;
    }

    pub fn add_people(&mut self, city_from: &CityHandle, city_to: &CityHandle, amount: usize) {
        let people_transit: PeopleTransit = PeopleTransit{
            city_from: city_from.clone(),
            city_to: city_to.clone(),
            amount: amount,
            peoples_bus: None,
            got_off: false,
        };        
        self.people.push(people_transit);
    }

    pub fn new_bus(&mut self, stops: &[&CityHandle]) {
        let mut ttf_roads: Vec<usize> = Vec::new();
        for i in 0..stops.len() - 1 {
            ttf_roads.push(self.get_ttf_road_from_cities(stops[i], stops[i + 1]));
        }

        let bus: Bus = Bus::new(stops, ttf_roads);
        self.buses.push(bus);
    }

    fn get_ttf_road_from_cities(&self, city_0: &CityHandle, city_1: &CityHandle) -> usize {
        let road_handle: RoadHandle = self.get_roadhandle_from_citieshandles(city_0, city_1);
        return self.roads[road_handle].duration;
    }

    fn get_roadhandle_from_citieshandles(&self, city_0: &CityHandle, city_1: &CityHandle) -> RoadHandle {

        for i  in 0..self.roads.len() {
            //If it is the road between the two specified cities
            if (self.roads[i].city_0 == *city_0 && self.roads[i].city_1 == *city_1) ||
               (self.roads[i].city_0 == *city_1 && self.roads[i].city_1 == *city_0) {
                return i;
            }
        };

        panic!("Looking for nonexistent road");
    }

    fn check_people_getting_off_on(&mut self) -> Option<Vec<BusStopEvent>> {

        let mut bus_changes = Vec::new();

        for i_bus in 0..self.buses.len() {
            
            let mut curr_bse: Option<BusStopEvent> = None;
            let bus = &mut self.buses[i_bus];
            
            if bus.is_at_stop() {

                if bus.got_done == false {
                    curr_bse = Some(BusStopEvent{ city: self.cities[bus.get_curr_stop()].clone(), got_on: 0, got_off: 0});
                }                
                if bus.cities_yet_to_visit().len() == 0 {
                    bus.got_done = true;
                }
            
                for people_transit in &mut self.people {                    
                    match people_transit.peoples_bus {
                        
                        Some(i_ppl_bus) => {
                            //Check if people are getting off
                    
                            if (i_ppl_bus == i_bus) && (people_transit.city_to == bus.get_curr_stop()) {
                                //Now we have people and the bus they are in who want to get off

                                //so we record this BusStopEvent
                                let off = people_transit.amount;
                                curr_bse = match curr_bse {
                                    Some(bse) => {
                                        Some(BusStopEvent{ city: bse.city, got_on: bse.got_on, got_off: bse.got_off + off })
                                    }
                                    None => {
                                        let city = self.cities[people_transit.city_to].clone();
                                        Some(BusStopEvent { city: city, got_on: 0, got_off: off })
                                    }
                                };

                                //and we get them off
                                people_transit.peoples_bus = None;
                                people_transit.got_off = true;    

                            }                                                
                        }

                        None => {                                      
                            //Check if people are getting on
                            let future_stops = bus.cities_yet_to_visit();

                            if (bus.get_curr_stop() == people_transit.city_from) && future_stops.contains(&people_transit.city_to) && !people_transit.got_off {
                                //so we record this BusStopEvent                                
                                
                                let on = people_transit.amount;                                
                                curr_bse = match curr_bse {
                                    Some(bse) => {
                                        Some(BusStopEvent{ city: bse.city, got_on: bse.got_on + on, got_off: bse.got_off })
                                    }
                                    None => {
                                        let city = self.cities[people_transit.city_from].clone();
                                        Some(BusStopEvent { city: city, got_on: on, got_off: 0 })
                                    }
                                };
                                        
                                //and we get them on
                                people_transit.peoples_bus = Some(i_bus);                                                        
                            }
                            

                        }
                    }
                }
            }

            if let Some(bse) = curr_bse {
                bus_changes.push(bse);
            }
            
        }

        if bus_changes.len() == 0 {
            return None;
        }
        else {
            return Some(bus_changes);
        }
        
    }
    
    pub fn execute(&mut self, time_units: usize) -> Vec<BusStopEvent> {
        let mut bus_stop_events: Vec<BusStopEvent> = Vec::new();

        //Move everything 1 by 1 time step until it was done
        for _i in 0..time_units {

            match self.check_people_getting_off_on() {
                Some(mut bus_changes)=> {
                    bus_stop_events.append(&mut bus_changes);
                }
                None => {}
            }

            //Move busses by 1 step
            for bus in &mut self.buses {
                bus.step();
            }
        }
        
        return bus_stop_events;
    }
}