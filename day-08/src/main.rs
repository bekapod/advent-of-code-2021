use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let entries = day_08::read_entries_from_file(filename);

    let first = Instant::now();
    println!(
        "there are {} unique segments, time: {:.2?}",
        day_08::get_number_of_unique_segments_in_output_from_entries(&entries),
        first.elapsed()
    );

    let second = Instant::now();
    println!(
        "there sum of all outputs is {}, time: {:.2?}",
        day_08::decode_and_sum_entries(&entries),
        second.elapsed()
    );
}
