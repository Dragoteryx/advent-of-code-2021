use std::str::FromStr;
use std::fs;

// commands

enum Command {
  Forward(u32),
  Up(u32),
  Down(u32)
}

impl FromStr for Command {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, ()> {
    let mut iter = s.split_whitespace();
    if let (Some(cmd), Some(Ok(arg))) = (iter.next(), iter.next().map(|s| s.parse())) {
      match cmd {
        "forward" => Ok(Self::Forward(arg)),
        "up" => Ok(Self::Up(arg)),
        "down" => Ok(Self::Down(arg)),
        _ => Err(())
      }
    } else {
      Err(())
    }
  }
}

fn commands() -> Vec<Command> {
  fs::read_to_string("src/input.txt")
    .expect("Failed to read input")
    .lines()
    .map(|s| s.parse().expect("Failed to parse command"))
    .collect()
}

// part 1

fn part1() -> u32 {
  let commands = commands();
  let mut pos = (0, 0);
  for command in commands {
    match command {
      Command::Forward(n) => pos.0 += n,
      Command::Up(n) => pos.1 -= n,
      Command::Down(n) => pos.1 += n
    }
  }
  pos.0 * pos.1
}

#[test]
fn part1_test() {
  assert_eq!(part1(), 2150351);
}

// part 2

fn part2() -> u32 {
  let commands = commands();
  let mut pos = (0, 0);
  let mut aim = 0;
  for command in commands {
    match command {
      Command::Up(n) => aim -= n,
      Command::Down(n) => aim += n,
      Command::Forward(n) => {
        pos.0 += n;
        pos.1 += n*aim;
      }
    }
  }
  pos.0 * pos.1
}

#[test]
fn part2_test() {
  assert_eq!(part2(), 1842742223);
}

// main

fn main() {
  println!("--- day 2 ---");
  println!("part 1: {}", part1());
  println!("part 2: {}", part2());
}
