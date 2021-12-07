#![warn(clippy::all, clippy::pedantic)]
use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

#[derive(Debug, Clone)]
pub struct Crabs {
  positions: Vec<i32>,
}

impl Crabs {
  pub fn new(filename: impl AsRef<Path>) -> Crabs {
    let file = File::open(filename).expect("file doesn't exist");
    let reader = BufReader::new(file);

    Crabs {
      positions: reader
        .lines()
        .next()
        .expect("apparently there is no first item")
        .expect("is it normal to have a result wrapped in an option??")
        .split(',')
        .fold(vec![], |positions, position| {
          [
            positions,
            vec![position.parse::<i32>().expect("not a number")],
          ]
          .concat()
        }),
    }
  }

  fn average(numbers: &[i32]) -> f64 {
    let sum: i32 = numbers.iter().sum();
    f64::from(sum) / (numbers.len() as f64)
  }

  #[must_use]
  pub fn find_lowest_constant_fuel_cost(&self) -> i32 {
    let mut sorted = self.positions.clone();
    sorted.sort_unstable();
    let middle = sorted.len() / 2;

    sorted.iter().fold(0, |fuel_cost, current_position| {
      fuel_cost
        + (current_position
          - (if sorted.len() % 2 == 0 {
            Crabs::average(&sorted[(middle - 1)..(middle + 1)]) as i32
          } else {
            sorted[middle]
          }))
        .abs()
    })
  }

  #[must_use]
  pub fn find_lowest_quadratic_fuel_cost(&self) -> i32 {
    fn cost_to_position(positions: &[i32], position: i32) -> i32 {
      positions.iter().fold(0, |total, current_position| {
        total + Crabs::calculate_quadratic_fuel_cost_for_move((current_position - position).abs())
      })
    }

    let average = Crabs::average(&self.positions);
    let costs = vec![
      cost_to_position(&self.positions, average.ceil() as i32),
      cost_to_position(&self.positions, average.floor() as i32),
    ];
    *costs
      .iter()
      .min()
      .expect("apparently there is no minimum in this list of numbers")
  }

  fn calculate_quadratic_fuel_cost_for_move(distance: i32) -> i32 {
    (distance * distance + distance) / 2
  }
}

// let realFuelCost = data.reduce((acc, v) => acc + tri(Math.abs(mean - v)), 0);

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn create_new_crabs_example() {
    assert_eq!(
      Crabs::new("example.txt").positions,
      [16, 1, 2, 0, 4, 2, 7, 1, 2, 14]
    );
  }

  #[test]
  fn find_fuel_cost_constant_example() {
    assert_eq!(
      Crabs::new("example.txt").find_lowest_constant_fuel_cost(),
      37
    );
  }

  #[test]
  fn find_fuel_cost_constant_input() {
    assert_eq!(
      Crabs::new("input.txt").find_lowest_constant_fuel_cost(),
      344_735
    );
  }

  #[test]
  fn find_fuel_cost_quadratic_example() {
    assert_eq!(
      Crabs::new("example.txt").find_lowest_quadratic_fuel_cost(),
      168
    );
  }

  #[test]
  fn find_fuel_cost_quadratic_input() {
    assert_eq!(
      Crabs::new("input.txt").find_lowest_quadratic_fuel_cost(),
      96_798_233
    );
  }
}
