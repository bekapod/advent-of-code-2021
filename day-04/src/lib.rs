#![warn(clippy::all, clippy::pedantic)]
use std::collections::HashSet;
use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

#[derive(Clone, Debug)]
pub struct Board {
  rows: Vec<HashSet<u8>>,
  columns: Vec<HashSet<u8>>,
  called_numbers: HashSet<u8>,
  winning_number: Option<u8>,
}

impl Board {
  fn new(rows: Vec<HashSet<u8>>, columns: Vec<HashSet<u8>>) -> Self {
    Board {
      rows,
      columns,
      called_numbers: HashSet::new(),
      winning_number: None,
    }
  }

  fn win(&mut self, called_numbers: HashSet<u8>, number: u8) {
    self.called_numbers = called_numbers;
    self.winning_number = Some(number);
  }

  #[must_use]
  pub fn get_score(&self) -> Option<u32> {
    if let Some(winning_number) = self.winning_number {
      let score = self.rows.iter().fold(0, |x, row| {
        x + row.iter().fold(0, |y, cell| {
          if self.called_numbers.contains(cell) {
            return y;
          }

          y + u32::from(*cell)
        })
      }) * u32::from(winning_number);

      return Some(score);
    }

    None
  }
}

pub fn boards_from_file(filename: impl AsRef<Path>) -> (Vec<u8>, Vec<Board>) {
  let file = File::open(filename).expect("file doesn't exist");
  let reader = BufReader::new(file);
  let mut lines = reader
    .lines()
    .map(|line| line.expect("couldn't parse line"));

  let numbers = lines
    .by_ref()
    .take(1)
    .next()
    .expect("couldn't get first line")
    .split(',')
    .map(|n| n.parse::<u8>().expect("not a number"))
    .collect::<Vec<u8>>();

  let all_boards = lines
    .by_ref()
    .skip(1)
    .filter(|l| !l.is_empty())
    .collect::<Vec<String>>()
    .chunks(5)
    .fold(Vec::new(), |boards, chunk| {
      let mut rows = Vec::with_capacity(5);
      let mut columns: [HashSet<u8>; 5] = [
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
        HashSet::new(),
      ];

      for row in chunk {
        let mut row_set = HashSet::new();
        let cells = row
          .split(' ')
          .filter(|cell| !cell.is_empty())
          .map(|cell| cell.trim().parse::<u8>().expect("non-number found"));

        for (column_number, cell) in cells.enumerate() {
          row_set.insert(cell);
          columns[column_number].insert(cell);
        }
        rows.push(row_set);
      }

      [boards, vec![Board::new(rows, columns.to_vec())]].concat()
    });

  (numbers, all_boards)
}

#[must_use]
pub fn play(numbers: &[u8], boards: &[Board]) -> Vec<Board> {
  let mut called_numbers = HashSet::from([numbers[0], numbers[1], numbers[2], numbers[3]]);
  let mut winning_boards = vec![];
  let mut my_boards = [boards].concat();

  for called_number in numbers.iter().skip(4) {
    called_numbers.insert(*called_number);
    for board in &mut my_boards {
      if board.winning_number.is_none() {
        for row in &board.rows {
          if row.is_subset(&called_numbers) {
            board.win(called_numbers.clone(), *called_number);
            winning_boards.push(board.clone());
            break;
          }
        }

        for column in &board.columns {
          if column.is_subset(&called_numbers) {
            board.win(called_numbers.clone(), *called_number);
            winning_boards.push(board.clone());
            break;
          }
        }
      }
    }
  }

  winning_boards
}
