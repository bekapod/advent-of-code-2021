use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let number_of_steps = &args[2]
        .parse::<u32>()
        .expect("number of steps must be a number");

    let first = Instant::now();
    println!(
        "number of steps: {}, flashes: {}, time: {:.2?}",
        number_of_steps,
        day_11::OctopusCavern::new(filename).play_steps(*number_of_steps),
        first.elapsed()
    );

    let second = Instant::now();
    println!(
        "brightest step: {}, time: {:.2?}",
        day_11::OctopusCavern::new(filename).find_brightest_step(),
        second.elapsed()
    );
}
