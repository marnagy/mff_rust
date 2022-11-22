mod solution;

use solution::Simulation;

fn main() {
    let mut simulation = Simulation::new();

    let prg = simulation.new_city("Prague");
    let brn = simulation.new_city("Brno");

    let d1 = simulation.new_road(&prg, &brn, 120);

    simulation.new_bus(&[&prg, &brn]);

    simulation.add_people(&prg, &brn, 50);

    println!("Simulation: {:?}", simulation);
}
