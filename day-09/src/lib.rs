#![warn(clippy::all, clippy::pedantic)]
use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

#[derive(Debug, PartialEq)]
pub struct Floor {
  map: Vec<Vec<u8>>,
}

impl Floor {
  pub fn new(filename: impl AsRef<Path>) -> Floor {
    let file = File::open(filename).expect("file doesn't exist");
    let reader = BufReader::new(file);

    Floor {
      map: reader
        .lines()
        .map(|line| {
          line
            .expect("could not parse line")
            .split("")
            .filter(|height| !height.is_empty())
            .map(|height| height.parse::<u8>().expect("not a number"))
            .collect()
        })
        .collect(),
    }
  }

  fn is_low_point(&self, row: &[u8], y: usize, x: usize, height: u8) -> bool {
    if x > 0 && row[x - 1] <= height {
      return false;
    }
    if x < row.len() - 1 && row[x + 1] <= height {
      return false;
    }
    if y > 0 && self.map[y - 1][x] <= height {
      return false;
    }
    if y < self.map.len() - 1 && self.map[y + 1][x] <= height {
      return false;
    }

    true
  }

  fn find_all_low_points(&self) -> Vec<u8> {
    let mut low_points = vec![];
    for (y, row) in self.map.iter().enumerate() {
      for (x, height) in row.iter().enumerate() {
        if self.is_low_point(row, y, x, *height) {
          low_points.push(*height);
        }
      }
    }

    low_points
  }

  fn find_full_basin(&self, mut basin: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let (y, x) = basin[basin.len() - 1];
    let height = self.map[y][x];

    if x > 0 {
      let tile_before_pos = (y, x - 1);
      let tile_before = self.map[tile_before_pos.0][tile_before_pos.1];
      if tile_before >= height && tile_before != 9 && !basin.contains(&tile_before_pos) {
        basin.push(tile_before_pos);
        basin = self.find_full_basin(basin);
      }
    }
    if x < self.map[y].len() - 1 {
      let tile_after_pos = (y, x + 1);
      let tile_after = self.map[tile_after_pos.0][tile_after_pos.1];
      if tile_after >= height && tile_after != 9 && !basin.contains(&tile_after_pos) {
        basin.push(tile_after_pos);
        basin = self.find_full_basin(basin);
      }
    }
    if y > 0 {
      let tile_prev_row_pos = (y - 1, x);
      let tile_prev_row = self.map[tile_prev_row_pos.0][tile_prev_row_pos.1];
      if tile_prev_row >= height && tile_prev_row != 9 && !basin.contains(&tile_prev_row_pos) {
        basin.push(tile_prev_row_pos);
        basin = self.find_full_basin(basin);
      }
    }
    if y < self.map.len() - 1 {
      let tile_next_row_pos = (y + 1, x);
      let tile_next_row = self.map[tile_next_row_pos.0][tile_next_row_pos.1];
      if tile_next_row >= height && tile_next_row != 9 && !basin.contains(&tile_next_row_pos) {
        basin.push(tile_next_row_pos);
        basin = self.find_full_basin(basin);
      }
    }

    basin
  }

  #[must_use]
  pub fn find_combined_size_of_biggest_basins(&self) -> usize {
    let mut basin_sizes = vec![];

    for (y, row) in self.map.iter().enumerate() {
      for (x, height) in row.iter().enumerate() {
        if self.is_low_point(row, y, x, *height) {
          let basin = self.find_full_basin(vec![(y, x)]);

          if basin.len() > 1 {
            basin_sizes.push(basin.len());
          }
        }
      }
    }

    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
  }

  #[must_use]
  pub fn get_risk_level(&self) -> u32 {
    self
      .find_all_low_points()
      .iter()
      .fold(0, |total, height| total + 1 + u32::from(*height))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_new_floor_example() {
    assert_eq!(
      Floor::new("example.txt"),
      Floor {
        map: vec![
          vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
          vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
          vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
          vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
          vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8]
        ]
      }
    );
  }

  #[test]
  fn find_all_low_points_example() {
    assert_eq!(
      Floor::new("example.txt").find_all_low_points(),
      vec![1, 0, 5, 5]
    );
  }

  #[test]
  fn get_risk_level_example() {
    assert_eq!(Floor::new("example.txt").get_risk_level(), 15);
  }

  #[test]
  fn find_combined_size_of_biggest_basins_example() {
    assert_eq!(
      Floor::new("example.txt").find_combined_size_of_biggest_basins(),
      1134
    );
  }
}
