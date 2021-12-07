use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let number_of_days = &args[2];
    let before = Instant::now();
    let mut sea = day_06::Sea::new(filename);
    sea.play(number_of_days.parse::<u32>().expect("not a number"));
    println!(
        "there are {} fish in the sea after {} days, time: {:.2?}",
        sea.how_many_fish_are_in_the_sea(),
        number_of_days,
        before.elapsed()
    );
}
