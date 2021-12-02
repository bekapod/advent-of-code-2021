use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

pub enum Command {
  Up(i32),
  Down(i32),
  Forward(i32),
}

pub fn commands_from_file(filename: impl AsRef<Path>) -> Vec<Command> {
  let file = File::open(filename).expect("file doesn't exist");
  let reader = BufReader::new(file);
  reader
    .lines()
    .map(|line| {
      let l = line.expect("couldn't parse line");
      let mut parts = l.split_whitespace();
      let direction = parts.next().expect("couldn't parse direction");
      let distance = parts
        .next()
        .expect("couldn't parse distance")
        .parse::<i32>()
        .expect("not a number");

      match direction {
        "up" => Command::Up(distance),
        "down" => Command::Down(distance),
        "forward" => Command::Forward(distance),
        _ => panic!("unknown direction"),
      }
    })
    .collect()
}

pub fn find_position(commands: &[Command]) -> i32 {
  let mut position: (i32, i32) = (0, 0);

  for command in commands {
    match command {
      Command::Up(direction) => position.1 -= direction,
      Command::Down(direction) => position.1 += direction,
      Command::Forward(direction) => position.0 += direction,
    }
  }
  position.0 * position.1
}

pub fn find_position_with_aim(commands: &[Command]) -> i32 {
  let mut position: (i32, i32, i32) = (0, 0, 0);

  for command in commands {
    match command {
      Command::Up(direction) => position.2 -= direction,
      Command::Down(direction) => position.2 += direction,
      Command::Forward(direction) => {
        position.0 += direction;
        position.1 += position.2 * direction
      }
    }
  }

  position.0 * position.1
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn find_position_example() {
    assert_eq!(
      find_position(&[
        Command::Forward(5),
        Command::Down(5),
        Command::Forward(8),
        Command::Up(3),
        Command::Down(8),
        Command::Forward(2)
      ]),
      150
    );
  }

  #[test]
  fn find_position_with_aim_example() {
    assert_eq!(
      find_position_with_aim(&[
        Command::Forward(5),
        Command::Down(5),
        Command::Forward(8),
        Command::Up(3),
        Command::Down(8),
        Command::Forward(2)
      ]),
      900
    );
  }
}
