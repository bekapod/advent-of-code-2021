use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let (numbers, boards) = day_04::boards_from_file(filename);
    let winning_boards = day_04::play(&numbers, &boards);
    println!(
        "first winner score: {} | last winner score: {}",
        winning_boards
            .first()
            .expect("couldn't get first board")
            .get_score()
            .expect("first board did not win"),
        winning_boards
            .last()
            .expect("couldn't get last board")
            .get_score()
            .expect("last board did not win")
    );
}
