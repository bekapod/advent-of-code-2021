use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let before = Instant::now();
    let diagram = day_05::Diagram::new(filename);
    let number_of_dangerous_areas = diagram.get_number_of_dangerous_areas();
    println!(
        "number of dangerous areas: {}, time: {:.2?}",
        number_of_dangerous_areas,
        before.elapsed()
    );
}
