use std::collections::BTreeMap;
use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
  let file = File::open(filename).expect("file doesn't exist");
  let reader = BufReader::new(file);
  reader
    .lines()
    .map(|line| line.expect("couldn't parse line"))
    .collect()
}

fn bits_from_lines(lines: Vec<String>) -> Vec<Vec<u8>> {
  let init: Vec<Vec<u8>> = Vec::new();
  lines.iter().fold(init, |columns, line| {
    let parts = line.chars();
    let mut result = columns.clone();
    for (idx, column) in parts.enumerate() {
      let bit = column.to_digit(10).expect("not a number") as u8;
      if result.len() <= idx {
        result.push(Vec::new());
      }
      result[idx].push(bit)
    }

    result
  })
}

fn find_occurrences_in_column(column: Vec<u8>) -> BTreeMap<u8, i32> {
  let mut counts = BTreeMap::new();
  for bit in column {
    *counts.entry(bit).or_insert(0) += 1;
  }

  counts
}

fn calculate_gamma_and_epsilon_rate(bits: Vec<Vec<u8>>) -> (u32, u32) {
  let mut gamma_rate = String::new();
  let mut epsilon_rate = String::new();

  for column in bits {
    let counts = find_occurrences_in_column(column);

    gamma_rate += &counts
      .clone()
      .into_iter()
      .max_by_key(|&(_, count)| count)
      .expect("empty")
      .0
      .to_string();

    epsilon_rate += &counts
      .clone()
      .into_iter()
      .min_by_key(|&(_, count)| count)
      .expect("empty")
      .0
      .to_string();
  }

  (
    isize::from_str_radix(&gamma_rate, 2).expect("not a binary number") as u32,
    isize::from_str_radix(&epsilon_rate, 2).expect("not a binary number") as u32,
  )
}

fn calculate_oxygen_and_co2_rating(lines: Vec<String>) -> (u32, u32) {
  let mut oxygen_considerations = lines.clone();
  let mut co2_considerations = lines.clone();

  enum ConsiderationMode {
    MostCommon,
    LeastCommon,
  }

  let filter_considerations =
    |considerations: Vec<String>, idx: usize, mode: ConsiderationMode| -> Vec<String> {
      let columns = bits_from_lines(considerations.clone());
      let counts = find_occurrences_in_column(columns[idx].clone());
      let most_common = &counts
        .clone()
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .expect("empty")
        .0
        .to_string();
      let least_common = &counts
        .clone()
        .into_iter()
        .min_by_key(|&(_, count)| count)
        .expect("empty")
        .0
        .to_string();
      let mut bit: &String;

      match mode {
        ConsiderationMode::MostCommon => {
          bit = most_common;
          if most_common == least_common && most_common == "0" {
            bit = least_common;
          }
        }
        ConsiderationMode::LeastCommon => {
          bit = least_common;
          if most_common == least_common && least_common == "1" {
            bit = most_common;
          }
        }
      }

      considerations
        .into_iter()
        .filter(|l| {
          l.chars().nth(idx).expect("could not parse number")
            == bit.parse::<char>().expect("could not parse bit")
        })
        .collect()
    };

  let mut idx = 0;
  while oxygen_considerations.len() > 1 {
    oxygen_considerations =
      filter_considerations(oxygen_considerations, idx, ConsiderationMode::MostCommon);
    idx += 1;
  }
  let mut idx = 0;
  while co2_considerations.len() > 1 {
    co2_considerations =
      filter_considerations(co2_considerations, idx, ConsiderationMode::LeastCommon);
    idx += 1;
  }

  (
    isize::from_str_radix(&oxygen_considerations[0], 2).expect("not a binary number") as u32,
    isize::from_str_radix(&co2_considerations[0], 2).expect("not a binary number") as u32,
  )
}

pub fn calculate_power_consumption(lines: Vec<String>) -> u32 {
  let (gamma_rate, epsilon_rate) = calculate_gamma_and_epsilon_rate(bits_from_lines(lines));
  gamma_rate * epsilon_rate
}

pub fn calculate_life_support_rating(lines: Vec<String>) -> u32 {
  let (oxygen_rating, co2_rating) = calculate_oxygen_and_co2_rating(lines);
  oxygen_rating * co2_rating
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn calculate_gamma_and_epsilon_rate_example() {
    assert_eq!(
      calculate_gamma_and_epsilon_rate(vec![
        vec![0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0],
        vec![0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1],
        vec![0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0]
      ]),
      (22, 9)
    );
  }

  #[test]
  fn calculate_power_consumption_example() {
    assert_eq!(
      calculate_power_consumption(vec![
        "00100".to_string(),
        "11110".to_string(),
        "10110".to_string(),
        "10111".to_string(),
        "10101".to_string(),
        "01111".to_string(),
        "00111".to_string(),
        "11100".to_string(),
        "10000".to_string(),
        "11001".to_string(),
        "00010".to_string(),
        "01010".to_string(),
      ]),
      198
    );
  }

  #[test]
  fn calculate_oxygen_and_co2_rating_example() {
    assert_eq!(
      calculate_oxygen_and_co2_rating(vec![
        "00100".to_string(),
        "11110".to_string(),
        "10110".to_string(),
        "10111".to_string(),
        "10101".to_string(),
        "01111".to_string(),
        "00111".to_string(),
        "11100".to_string(),
        "10000".to_string(),
        "11001".to_string(),
        "00010".to_string(),
        "01010".to_string(),
      ]),
      (23, 10)
    );
  }

  #[test]
  fn calculate_life_support_rating_example() {
    assert_eq!(
      calculate_life_support_rating(vec![
        "00100".to_string(),
        "11110".to_string(),
        "10110".to_string(),
        "10111".to_string(),
        "10101".to_string(),
        "01111".to_string(),
        "00111".to_string(),
        "11100".to_string(),
        "10000".to_string(),
        "11001".to_string(),
        "00010".to_string(),
        "01010".to_string(),
      ]),
      230
    );
  }
}
