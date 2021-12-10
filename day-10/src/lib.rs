#![warn(clippy::all, clippy::pedantic)]
use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

const OPENING_CHARACTERS: [char; 4] = ['(', '{', '[', '<'];
const CLOSING_CHARACTERS: [char; 4] = [')', '}', ']', '>'];

#[derive(Debug, PartialEq)]
pub struct NavigationSubsystem {
  lines: Vec<String>,
}

impl NavigationSubsystem {
  pub fn new(filename: impl AsRef<Path>) -> NavigationSubsystem {
    let file = File::open(filename).expect("file doesn't exist");
    let reader = BufReader::new(file);

    NavigationSubsystem {
      lines: reader
        .lines()
        .map(|line| line.expect("could not parse line"))
        .collect(),
    }
  }

  fn find_invalid_character(line: &str) -> Option<char> {
    let mut stack = vec![];

    for character in line.chars() {
      if OPENING_CHARACTERS.contains(&character) {
        stack.push(character);
      }

      if CLOSING_CHARACTERS.contains(&character) {
        if let Some(opening_character) = stack.pop() {
          let opening_idx = OPENING_CHARACTERS
            .iter()
            .position(|c| c == &opening_character)
            .expect("invalid opening character");
          let closing_idx = CLOSING_CHARACTERS
            .iter()
            .position(|c| c == &character)
            .expect("invalid closing character");

          if opening_idx == closing_idx {
            continue;
          }

          return Some(character);
        }
      }
    }

    None
  }

  fn get_characters_needed_for_complete_line(line: &str) -> String {
    let mut stack = vec![];

    for character in line.chars() {
      if OPENING_CHARACTERS.contains(&character) {
        stack.push(character);
      }

      if CLOSING_CHARACTERS.contains(&character) {
        if let Some(opening_character) = stack.pop() {
          let opening_idx = OPENING_CHARACTERS
            .iter()
            .position(|c| c == &opening_character)
            .expect("invalid opening character");
          let closing_idx = CLOSING_CHARACTERS
            .iter()
            .position(|c| c == &character)
            .expect("invalid closing character");

          if opening_idx == closing_idx {
            continue;
          }
        }
      }
    }

    stack
      .iter()
      .rev()
      .fold(String::new(), |characters, character| {
        let idx = OPENING_CHARACTERS
          .iter()
          .position(|c| c == character)
          .expect("invalid opening character");
        characters + &CLOSING_CHARACTERS[idx].to_string()
      })
  }

  #[must_use]
  pub fn get_syntax_error_score(&self) -> u32 {
    let scores = std::collections::BTreeMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    self.lines.iter().fold(0, |score, line| {
      let invalid_character = NavigationSubsystem::find_invalid_character(line);

      if let Some(character) = invalid_character {
        match scores.get(&character) {
          Some(s) => {
            return score + s;
          }
          None => {
            return score;
          }
        }
      }

      score
    })
  }

  #[must_use]
  pub fn get_autocomplete_score(&self) -> u64 {
    let scores = std::collections::BTreeMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let mut line_scores = vec![];

    for line in &self.lines {
      if NavigationSubsystem::find_invalid_character(line).is_some() {
        continue;
      }

      line_scores.push(
        NavigationSubsystem::get_characters_needed_for_complete_line(line)
          .chars()
          .fold(0, |score, character| {
            score * 5
              + scores
                .get(&character)
                .expect("not a valid character for scoring")
          }),
      );
    }

    line_scores.sort_unstable();
    line_scores[line_scores.len() / 2]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn find_invalid_character_curly_bracket() {
    assert_eq!(
      NavigationSubsystem::find_invalid_character("{([(<{}[<>[]}>{[]{[(<()>"),
      Some('}')
    );
  }

  #[test]
  fn find_invalid_character_parenthesis() {
    assert_eq!(
      NavigationSubsystem::find_invalid_character("[[<[([]))<([[{}[[()]]]"),
      Some(')')
    );
  }

  #[test]
  fn find_invalid_character_square_bracket() {
    assert_eq!(
      NavigationSubsystem::find_invalid_character("[{[{({}]{}}([{[{{{}}([]"),
      Some(']')
    );
  }

  #[test]
  fn find_invalid_character_angle_bracket() {
    assert_eq!(
      NavigationSubsystem::find_invalid_character("<{([([[(<>()){}]>(<<{{"),
      Some('>')
    );
  }

  #[test]
  fn get_characters_needed_for_valid_line() {
    assert_eq!(
      NavigationSubsystem::get_characters_needed_for_complete_line("[({(<(())[]>[[{[]{<()<>>"),
      "}}]])})]"
    );
  }

  #[test]
  fn get_syntax_error_score_example() {
    assert_eq!(
      NavigationSubsystem::new("example.txt").get_syntax_error_score(),
      26397
    );
  }

  #[test]
  fn get_autocomplete_score_example() {
    assert_eq!(
      NavigationSubsystem::new("example.txt").get_autocomplete_score(),
      288_957
    );
  }
}
