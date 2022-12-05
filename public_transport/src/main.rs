mod solution;

use solution::Simulation;

// DONE: if multiple buses stop in 1 tick -> in order of buses
// DONE: if bus stops -> always generate events
// DONE: if people get both on and off, generate only 1 event

fn main() {
    let mut simulation = Simulation::new();

    let prg = simulation.new_city("Prague");
    let brn = simulation.new_city("Brno");
    let ba = simulation.new_city("Bratislava");

    let d1 = simulation.new_road(&prg, &brn, 10);
    let d2 = simulation.new_road(&brn, &ba, 10);
    let d3 = simulation.new_road(&ba, &prg, 35);

    //simulation.new_bus(&[&prg, &brn]);
    simulation.new_bus(&[&prg, &brn, &ba]);
    //simulation.new_bus(&[&ba, &prg, &brn]);

    simulation.add_people(&prg, &brn, 50);
    //simulation.add_people(&brn, &ba, 20);
    simulation.add_people(&prg, &ba, 30);

    // println!("Simulation: {:?}", simulation);

    for event in simulation.execute(50) {
        let name = event.city().name();
        let people_got_off = event.got_off();
        let people_got_on = event.got_on();
        println!("Name: {}", name);
        println!("On: {}", people_got_on);
        println!("Off: {}", people_got_off);
    }
}
