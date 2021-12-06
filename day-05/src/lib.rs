#![warn(clippy::all, clippy::pedantic)]
use std::collections::HashMap;
use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

#[derive(Clone, Debug)]
pub struct Diagram {
  value: HashMap<u16, HashMap<u16, u16>>,
}

impl Diagram {
  pub fn new(filename: impl AsRef<Path>) -> Diagram {
    let file = File::open(filename).expect("file doesn't exist");
    let reader = BufReader::new(file);
    let mut width = 0;
    let mut height = 0;

    let lines = reader
      .lines()
      .map(|line| line.expect("couldn't parse line"))
      .map(|line| {
        line
          .split(" -> ")
          .map(|parts| {
            parts
              .split(',')
              .map(|n| n.parse::<u16>().expect("not a number"))
              .collect::<Vec<u16>>()
          })
          .map(|set| {
            let x = *set.first().expect("no x coordinate");
            let y = *set.last().expect("no y coordinate");
            if x > width {
              width = x;
            }
            if y > height {
              height = y;
            }
            (x, y)
          })
          .collect::<Vec<(u16, u16)>>()
      })
      .collect::<Vec<Vec<(u16, u16)>>>();

    let mut diagram = Diagram {
      value: HashMap::new(),
    };

    diagram.fill_empty(width, height);

    for line in lines {
      let (start_x, start_y) = *line.first().expect("no start coordinate");
      let (end_x, end_y) = *line.last().expect("no end coordinate");

      diagram.plot_line((start_x, end_x), (start_y, end_y));
    }

    diagram
  }

  fn fill_empty(&mut self, width: u16, height: u16) {
    let row = (0..=width).map(|i| (i, 0)).collect::<HashMap<u16, u16>>();

    for i in 0..=height {
      self.value.entry(i).or_insert_with(|| row.clone());
    }
  }

  fn plot_line(&mut self, x: (u16, u16), y: (u16, u16)) {
    let (start_x, end_x) = x;
    let (start_y, end_y) = y;

    if is_horizontal_line((start_x, end_x), (start_y, end_y)) {
      self.plot_straight_line((start_x, end_x), (start_y, end_y));
    }

    if is_perfect_diagonal_line((start_x, end_x), (start_y, end_y)) {
      self.plot_diagonal_line((start_x, end_x), (start_y, end_y));
    }
  }

  fn plot_straight_line(&mut self, x: (u16, u16), y: (u16, u16)) {
    let (start_x, end_x) = x;
    let (start_y, end_y) = y;

    for i in if start_y > end_y {
      end_y..=start_y
    } else {
      start_y..=end_y
    } {
      for j in if start_x > end_x {
        end_x..=start_x
      } else {
        start_x..=end_x
      } {
        self.plot_point(j, i);
      }
    }
  }

  fn plot_diagonal_line(&mut self, x: (u16, u16), y: (u16, u16)) {
    let (start_x, end_x) = x;
    let (start_y, end_y) = y;
    let ys = if start_y > end_y {
      (end_y..=start_y).rev().collect::<Vec<u16>>()
    } else {
      (start_y..=end_y).collect::<Vec<u16>>()
    };
    let xs = if start_x > end_x {
      (end_x..=start_x).rev().collect::<Vec<u16>>()
    } else {
      (start_x..=end_x).collect::<Vec<u16>>()
    };

    for (idx, i) in xs.iter().enumerate() {
      self.plot_point(*i, ys[idx]);
    }
  }

  fn plot_point(&mut self, x: u16, y: u16) {
    self.value.entry(y).and_modify(|c| {
      c.entry(x).and_modify(|r| {
        *r += 1;
      });
    });
  }

  #[must_use]
  pub fn get_number_of_dangerous_areas(self) -> u16 {
    self.value.iter().fold(0, |dangerous_area_count, (_, row)| {
      dangerous_area_count
        + row.iter().fold(0, |c, (_, column)| {
          if *column > 1 {
            return c + 1;
          }

          c
        })
    })
  }
}

fn is_horizontal_line(x: (u16, u16), y: (u16, u16)) -> bool {
  let (start_x, end_x) = x;
  let (start_y, end_y) = y;
  start_x == end_x || start_y == end_y
}

fn is_perfect_diagonal_line(x: (u16, u16), y: (u16, u16)) -> bool {
  let (start_x, end_x) = x;
  let (start_y, end_y) = y;
  let diff_x = if start_x > end_x {
    start_x - end_x
  } else {
    end_x - start_x
  };
  let diff_y = if start_y > end_y {
    start_y - end_y
  } else {
    end_y - start_y
  };
  diff_x == diff_y
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn get_number_of_dangerous_areas_example() {
    assert_eq!(
      Diagram::new("example.txt").get_number_of_dangerous_areas(),
      12
    );
  }

  #[test]
  fn get_number_of_dangerous_areas_actual() {
    assert_eq!(
      Diagram::new("input.txt").get_number_of_dangerous_areas(),
      21038
    );
  }
}
