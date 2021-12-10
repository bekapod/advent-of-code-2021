use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let navigation_subsystem = day_10::NavigationSubsystem::new(filename);

    let first = Instant::now();
    println!(
        "syntax score: {}, time: {:.2?}",
        navigation_subsystem.get_syntax_error_score(),
        first.elapsed()
    );

    let second = Instant::now();
    println!(
        "autocomplete score: {}, time: {:.2?}",
        navigation_subsystem.get_autocomplete_score(),
        second.elapsed()
    );
}
