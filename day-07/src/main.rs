use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let crabs = day_07::Crabs::new(filename);

    let first = Instant::now();
    println!(
        "moving the energetic crabs costs {} fuel, time: {:.2?}",
        crabs.find_lowest_constant_fuel_cost(),
        first.elapsed()
    );

    let second = Instant::now();
    println!(
        "moving the lazy crabs costs {} fuel, time: {:.2?}",
        crabs.find_lowest_quadratic_fuel_cost(),
        second.elapsed()
    );
}
