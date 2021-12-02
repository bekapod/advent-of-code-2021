use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let commands = day_02::commands_from_file(filename);
    println!("Part 1: {:?}", day_02::find_position(&commands));
    println!("Part 2: {:?}", day_02::find_position_with_aim(&commands));
}
