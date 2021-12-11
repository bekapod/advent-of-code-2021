#![warn(clippy::all, clippy::pedantic)]
use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

const MAX_OCTOPUS_ENERGY: u8 = 9;

#[derive(Debug, PartialEq)]
pub struct OctopusCavern(Vec<Vec<u8>>);

impl OctopusCavern {
  pub fn new(filename: impl AsRef<Path>) -> OctopusCavern {
    let file = File::open(filename).expect("file doesn't exist");
    let reader = BufReader::new(file);

    OctopusCavern {
      0: reader
        .lines()
        .map(|line| {
          line
            .expect("could not parse line")
            .split("")
            .filter(|position| !position.is_empty())
            .map(|position| position.parse::<u8>().expect("position was not a number"))
            .collect()
        })
        .collect(),
    }
  }

  fn give_energy(&mut self, x: usize, y: usize) -> u32 {
    let mut result = 0;
    self.0[y][x] += 1;

    // flash!
    if self.0[y][x] == MAX_OCTOPUS_ENERGY + 1 {
      result += 1;

      if y > 0 {
        result += self.give_energy(x, y - 1);
        if x > 0 {
          result += self.give_energy(x - 1, y - 1);
        }
        if x < self.0[y].len() - 1 {
          result += self.give_energy(x + 1, y - 1);
        }
      }
      if y < self.0.len() - 1 {
        result += self.give_energy(x, y + 1);
        if x > 0 {
          result += self.give_energy(x - 1, y + 1);
        }
        if x < self.0[y].len() - 1 {
          result += self.give_energy(x + 1, y + 1);
        }
      }
      if x > 0 {
        result += self.give_energy(x - 1, y);
      }
      if x < self.0[y].len() - 1 {
        result += self.give_energy(x + 1, y);
      }
    }

    result
  }

  #[must_use]
  pub fn play_steps(&mut self, number_of_steps: u32) -> u32 {
    let mut number_of_flashes = 0;

    for _step in 1..=number_of_steps {
      for y in 0..self.0.len() {
        for x in 0..self.0[y].len() {
          number_of_flashes += self.give_energy(x, y);
        }
      }

      for y in 0..self.0.len() {
        for x in 0..self.0[y].len() {
          if self.0[y][x] > MAX_OCTOPUS_ENERGY {
            self.0[y][x] = 0;
          }
        }
      }
    }

    number_of_flashes
  }

  #[must_use]
  pub fn find_brightest_step(&mut self) -> u32 {
    let mut step = 1;

    loop {
      for y in 0..self.0.len() {
        for x in 0..self.0[y].len() {
          self.give_energy(x, y);
        }
      }

      let mut all_flashed = true;
      for y in 0..self.0.len() {
        for x in 0..self.0[y].len() {
          if self.0[y][x] > MAX_OCTOPUS_ENERGY {
            self.0[y][x] = 0;
          } else {
            all_flashed = false;
          }
        }
      }

      if all_flashed {
        break;
      }

      step += 1;
    }

    step
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn two_step_simple() {
    assert_eq!(OctopusCavern::new("simple-example.txt").play_steps(2), 9);
  }

  #[test]
  fn two_steps_example() {
    assert_eq!(OctopusCavern::new("example.txt").play_steps(2), 35);
  }

  #[test]
  fn three_steps_example() {
    assert_eq!(OctopusCavern::new("example.txt").play_steps(3), 80);
  }

  #[test]
  fn four_steps_example() {
    assert_eq!(OctopusCavern::new("example.txt").play_steps(4), 96);
  }

  #[test]
  fn five_steps_example() {
    assert_eq!(OctopusCavern::new("example.txt").play_steps(5), 104);
  }

  #[test]
  fn ten_steps_example() {
    assert_eq!(OctopusCavern::new("example.txt").play_steps(10), 204);
  }

  #[test]
  fn one_hundred_steps_example() {
    assert_eq!(OctopusCavern::new("example.txt").play_steps(100), 1656);
  }

  #[test]
  fn one_hundred_steps_input() {
    assert_eq!(OctopusCavern::new("input.txt").play_steps(100), 1697);
  }

  #[test]
  fn find_brightest_step_example() {
    assert_eq!(OctopusCavern::new("example.txt").find_brightest_step(), 195);
  }

  #[test]
  fn find_brightest_step_input() {
    assert_eq!(OctopusCavern::new("input.txt").find_brightest_step(), 344);
  }
}
