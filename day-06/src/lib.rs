#![warn(clippy::all, clippy::pedantic)]
use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

#[derive(Debug)]
pub struct Sea {
  population: [usize; 9],
}

impl Sea {
  pub fn new(filename: impl AsRef<Path>) -> Sea {
    let file = File::open(filename).expect("file doesn't exist");
    let reader = BufReader::new(file);

    Sea {
      population: reader
        .lines()
        .next()
        .expect("apparently there is no first item")
        .expect("is it normal to have a result wrapped in an option??")
        .split(',')
        .fold([0; 9], |population, timer| {
          let mut result = population;
          let age = timer.parse::<usize>().expect("not a number");
          result[age] += 1;
          result
        }),
    }
  }

  pub fn play(&mut self, days: u32) {
    for _ in 0..days {
      self.cycle();
    }
  }

  #[must_use]
  pub fn how_many_fish_are_in_the_sea(self) -> usize {
    self.population.iter().sum()
  }

  fn cycle(&mut self) {
    let mut new_population = self.population;
    new_population.rotate_left(1);
    new_population[6] += self.population[0];
    self.population = new_population;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn create_new_sea_example() {
    assert_eq!(
      Sea::new("example.txt").population,
      [0, 1, 1, 2, 1, 0, 0, 0, 0]
    );
  }

  #[test]
  fn simple_cycle_example() {
    let mut sea = Sea::new("example.txt");
    sea.cycle();
    assert_eq!(sea.population, [1, 1, 2, 1, 0, 0, 0, 0, 0]);
    assert_eq!(sea.how_many_fish_are_in_the_sea(), 5);
  }

  #[test]
  fn cycle_where_new_fish_are_born_example() {
    let mut sea = Sea::new("example.txt");
    sea.cycle();
    sea.cycle();
    sea.cycle();
    assert_eq!(sea.population, [2, 1, 0, 0, 0, 1, 1, 1, 1]);
    assert_eq!(sea.how_many_fish_are_in_the_sea(), 7);
  }

  #[test]
  fn play_example_short() {
    let mut sea = Sea::new("example.txt");
    sea.play(18);
    assert_eq!(sea.how_many_fish_are_in_the_sea(), 26);
  }

  #[test]
  fn play_example_long() {
    let mut sea = Sea::new("example.txt");
    sea.play(80);
    assert_eq!(sea.how_many_fish_are_in_the_sea(), 5934);
  }

  #[test]
  fn play_input_long() {
    let mut sea = Sea::new("input.txt");
    sea.play(80);
    assert_eq!(sea.how_many_fish_are_in_the_sea(), 393_019);
  }

  #[test]
  fn play_input_longer() {
    let mut sea = Sea::new("input.txt");
    sea.play(256);
    assert_eq!(sea.how_many_fish_are_in_the_sea(), 1_757_714_216_975);
  }
}
