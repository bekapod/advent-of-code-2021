use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let lines = day_01::lines_from_file(filename);
    println!(
        "Part 1: {:?}",
        day_01::find_simple_number_of_increases(lines.clone())
    );
    println!(
        "Part 2: {:?}",
        day_01::find_sliding_number_of_increases(lines.clone())
    );
}
