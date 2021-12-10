use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let floor = day_09::Floor::new(filename);

    let first = Instant::now();
    println!(
        "risk level: {}, time: {:.2?}",
        floor.get_risk_level(),
        first.elapsed()
    );

    let second = Instant::now();
    println!(
        "the combined size of the biggest basins is: {}, time: {:.2?}",
        floor.find_combined_size_of_biggest_basins(),
        second.elapsed()
    );
}
