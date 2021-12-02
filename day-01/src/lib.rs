use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<u32> {
  let file = File::open(filename).expect("file doesn't exist");
  let reader = BufReader::new(file);
  reader
    .lines()
    .map(|line| {
      line
        .expect("couldn't parse line")
        .parse::<u32>()
        .expect("not a number")
    })
    .collect()
}

pub fn find_simple_number_of_increases(numbers: Vec<u32>) -> u32 {
  let mut prev_number: u32 = 0;
  let mut answer: u32 = 0;

  for (i, number) in numbers.iter().enumerate() {
    if i > 0 && *number > prev_number {
      answer += 1
    }

    prev_number = *number
  }

  answer
}

pub fn find_sliding_number_of_increases(numbers: Vec<u32>) -> u32 {
  let windows = numbers.windows(3).map(|w| w.iter().sum()).collect();
  find_simple_number_of_increases(windows)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn simple_single_number() {
    assert_eq!(find_simple_number_of_increases(vec![1]), 0);
  }

  #[test]
  fn simple_more_numbers() {
    assert_eq!(find_simple_number_of_increases(vec![1, 2, 3, 4, 5]), 4);
  }

  #[test]
  fn simple_with_decreases() {
    assert_eq!(find_simple_number_of_increases(vec![4, 2, 1, 3, 5]), 2);
  }

  #[test]
  fn simple_with_only_decreases() {
    assert_eq!(find_simple_number_of_increases(vec![5, 4, 3, 2, 1]), 0);
  }

  #[test]
  fn sliding_single_number() {
    assert_eq!(find_sliding_number_of_increases(vec![1]), 0);
  }

  #[test]
  fn sliding_more_numbers() {
    assert_eq!(find_sliding_number_of_increases(vec![1, 2, 3, 4, 5]), 2);
  }

  #[test]
  fn sliding_with_decreases() {
    assert_eq!(find_sliding_number_of_increases(vec![4, 2, 1, 3, 5]), 1);
  }

  #[test]
  fn sliding_with_only_decreases() {
    assert_eq!(find_sliding_number_of_increases(vec![5, 4, 3, 2, 1]), 0);
  }

  #[test]
  fn sliding_from_example() {
    assert_eq!(
      find_sliding_number_of_increases(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
      5
    )
  }
}
