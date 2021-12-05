#![warn(clippy::all, clippy::pedantic)]
use std::collections::BTreeMap;
use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

#[derive(Clone, Debug)]
pub struct Diagram {
  value: BTreeMap<u16, BTreeMap<u16, u16>>,
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
          .split("->")
          .map(|parts| {
            parts
              .trim()
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
      value: BTreeMap::new(),
    };

    diagram.fill_empty(width, height);

    for line in lines {
      let (start_x, start_y) = line.first().expect("no start coordinate");
      let (end_x, end_y) = line.last().expect("no end coordinate");

      if start_x == end_x || start_y == end_y {
        diagram.plot_line((*start_x, *end_x), (*start_y, *end_y));
      }
    }

    diagram
  }

  fn fill_empty(&mut self, width: u16, height: u16) {
    for i in 0..=height {
      for j in 0..=width {
        self
          .value
          .entry(i)
          .or_insert_with(BTreeMap::new)
          .entry(j)
          .or_insert(0);
      }
    }
  }

  fn plot_line(&mut self, x: (u16, u16), y: (u16, u16)) {
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
        self.value.entry(i).and_modify(|c| {
          c.entry(j).and_modify(|r| {
            *r += 1;
          });
        });
      }
    }
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
