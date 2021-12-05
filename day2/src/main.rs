// commands

enum Command {
  Forward(u32),
  Up(u32),
  Down(u32)
}

fn commands() -> impl Iterator<Item = Command> {
  include_str!("input.txt")
    .lines()
    .map(|line| {
      let mut iter = line.split_whitespace();
      let cmd = iter.next().unwrap();
      let arg = iter.next().unwrap().parse().unwrap();
      match cmd {
        "forward" => Command::Forward(arg),
        "up" => Command::Up(arg),
        "down" => Command::Down(arg),
        _ => panic!("Invalid command")
      }
    })
}

// part 1

fn part1() -> u32 {
  let mut pos = (0, 0);
  for command in commands() {
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
  let mut pos = (0, 0);
  let mut aim = 0;
  for command in commands() {
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
