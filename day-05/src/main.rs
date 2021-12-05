use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let diagram = day_05::Diagram::new(filename);
    let number_of_dangerous_areas = diagram.get_number_of_dangerous_areas();
    println!("number of dangerous areas: {}", number_of_dangerous_areas);
}
