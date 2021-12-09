#![warn(clippy::all, clippy::pedantic)]
use std::collections::HashSet;
use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

#[derive(Debug, PartialEq)]
pub struct Entry {
  signal_patterns: Vec<String>,
  output: Vec<String>,
  all: Vec<String>,
}

impl Entry {
  #[must_use]
  pub fn new(entry: &str) -> Entry {
    let parts = entry
      .split(" | ")
      .map(|set_of_digits| set_of_digits.split(' ').map(String::from).collect())
      .collect::<Vec<Vec<String>>>();

    Entry {
      signal_patterns: parts[0].clone(),
      output: parts[1].clone(),
      all: parts.concat(),
    }
  }

  #[must_use]
  pub fn get_unique_segment_numbers_in_list(list: &[String]) -> Vec<&String> {
    list
      .iter()
      .filter(|number| matches!(number.len(), 2 | 3 | 4 | 7))
      .collect()
  }

  fn get_not_unique_segment_numbers_in_list(list: &[String]) -> Vec<&String> {
    list
      .iter()
      .filter(|number| matches!(number.len(), 5 | 6))
      .collect()
  }

  #[must_use]
  fn decode_numbers(&self) -> Vec<HashSet<String>> {
    let unique = Entry::get_unique_segment_numbers_in_list(&self.all);
    let not_unique = Entry::get_not_unique_segment_numbers_in_list(&self.all);
    let mut segments = vec![HashSet::new(); 10];

    for number in unique {
      let number_segments = number
        .split("")
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect::<HashSet<String>>();

      match number.len() {
        2 => {
          segments[1] = number_segments.clone();
          continue;
        }
        3 => {
          segments[7] = number_segments.clone();
          continue;
        }
        4 => {
          segments[4] = number_segments.clone();
          continue;
        }
        7 => {
          segments[8] = number_segments.clone();
          continue;
        }
        _ => (),
      }
    }

    for number in not_unique {
      let number_segments = number
        .split("")
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect::<HashSet<String>>();

      let seven = segments[7].clone();
      let diff_with_seven: HashSet<_> = number_segments.difference(&seven).collect();
      if diff_with_seven.len() == 4 {
        segments[6] = number_segments.clone();
        continue;
      }

      let one = segments[1].clone();
      let diff_with_one: HashSet<_> = number_segments.difference(&one).collect();
      if diff_with_one.len() == 3 {
        segments[3] = number_segments.clone();
        continue;
      }

      let four = segments[4].clone();
      let diff_with_four: HashSet<_> = number_segments.difference(&four).collect();
      if number_segments.len() == 6 && diff_with_four.len() == 3 {
        segments[0] = number_segments.clone();
        continue;
      }
      if number_segments.len() == 6 && diff_with_four.len() == 2 {
        segments[9] = number_segments.clone();
        continue;
      }
      if number_segments.len() == 5 && diff_with_four.len() == 3 {
        segments[2] = number_segments.clone();
        continue;
      }
      if number_segments.len() == 5 && diff_with_four.len() == 2 {
        segments[5] = number_segments.clone();
        continue;
      }
    }

    segments
  }

  #[must_use]
  pub fn decode_output(&self) -> u32 {
    let segments = self.decode_numbers();
    let mut decoded = String::new();

    for number in &self.output {
      let number_segments = number
        .split("")
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect::<HashSet<String>>();

      for (idx, segment) in segments.iter().enumerate() {
        if number_segments == *segment {
          decoded += &idx.to_string();
          break;
        }
      }
    }

    decoded
      .parse::<u32>()
      .expect("could not parse decoded as a number")
  }
}

pub fn read_entries_from_file(filename: impl AsRef<Path>) -> Vec<Entry> {
  let file = File::open(filename).expect("file doesn't exist");
  let reader = BufReader::new(file);

  reader
    .lines()
    .map(|line| Entry::new(&line.expect("could not read line")))
    .collect()
}

#[must_use]
pub fn get_number_of_unique_segments_in_output_from_entries(entries: &[Entry]) -> usize {
  entries.iter().fold(0, |total, entry| {
    total + Entry::get_unique_segment_numbers_in_list(&entry.output).len()
  })
}

#[must_use]
pub fn decode_and_sum_entries(entries: &[Entry]) -> u32 {
  entries
    .iter()
    .fold(0, |total, entry| total + entry.decode_output())
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn create_new_entry() {
    assert_eq!(
      Entry::new(
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"
      ),
      Entry {
        signal_patterns: vec![
          "be".to_string(),
          "cfbegad".to_string(),
          "cbdgef".to_string(),
          "fgaecd".to_string(),
          "cgeb".to_string(),
          "fdcge".to_string(),
          "agebfd".to_string(),
          "fecdb".to_string(),
          "fabcd".to_string(),
          "edb".to_string()
        ],
        output: vec![
          "fdgacbe".to_string(),
          "cefdb".to_string(),
          "cefbgd".to_string(),
          "gcbe".to_string()
        ],
        all: vec![
          "be".to_string(),
          "cfbegad".to_string(),
          "cbdgef".to_string(),
          "fgaecd".to_string(),
          "cgeb".to_string(),
          "fdcge".to_string(),
          "agebfd".to_string(),
          "fecdb".to_string(),
          "fabcd".to_string(),
          "edb".to_string(),
          "fdgacbe".to_string(),
          "cefdb".to_string(),
          "cefbgd".to_string(),
          "gcbe".to_string()
        ]
      }
    );
  }

  #[test]
  fn returns_unique_segment_numbers() {
    let entry = Entry::new(
      "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
    );
    assert_eq!(
      Entry::get_unique_segment_numbers_in_list(&entry.output),
      vec!["fdgacbe", "gcbe"]
    );
  }

  #[test]
  fn decodes_output() {
    assert_eq!(
      Entry::new(
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
      )
      .decode_output(),
      5353
    );
  }

  #[test]
  fn returns_total_number_of_unique_segment_numbers_in_entries() {
    let entries = read_entries_from_file("example.txt");
    assert_eq!(
      get_number_of_unique_segments_in_output_from_entries(&entries),
      26
    );
  }

  #[test]
  fn decode_and_sum_entries_example() {
    let entries = read_entries_from_file("example.txt");
    assert_eq!(decode_and_sum_entries(&entries), 61229);
  }
}
