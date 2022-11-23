mod solution;

use solution::Simulation;

fn main() {
    let mut simulation = Simulation::new();

    let prg = simulation.new_city("Prague");
    let brn = simulation.new_city("Brno");

    let d1 = simulation.new_road(&prg, &brn, 120);

    simulation.new_bus(&[&prg, &brn]);

    simulation.add_people(&prg, &brn, 50);

    // println!("Simulation: {:?}", simulation);

    for event in simulation.execute(125) {
        let name = event.city().name();
        let people_got_off = event.got_off();
        let people_got_on = event.got_on();
        println!("Name: {}", name);
        println!("On: {}", people_got_on);
        println!("Off: {}", people_got_off);
    }
}
