use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let columns = day_03::lines_from_file(filename);
    println!(
        "Part 1: {:?}",
        day_03::calculate_power_consumption(columns.clone())
    );
    println!(
        "Part 2: {:?}",
        day_03::calculate_life_support_rating(columns.clone())
    );
}
