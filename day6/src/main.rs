// util

fn lanternfishes() -> [u64; 9] {
  let iter = include_str!("input.txt")
    .split(',')
    .map(|s| s.parse::<usize>().unwrap());
  let mut fishes = [0; 9];
  for fish in iter {
    fishes[fish] += 1;
  }
  fishes
}

fn iter(n: u64) -> u64 {
  let mut fishes = lanternfishes();
  for _ in 0..n {
    let children = fishes[0];
    fishes[..7].rotate_left(1);
    fishes[6] += fishes[7];
    fishes[7] = fishes[8];
    fishes[8] = children;
  }
  fishes.iter().sum()
}

// part 1

fn part1() -> u64 {
  iter(80)
}

#[test]
fn part1_test() {
  assert_eq!(part1(), 351188);
}

// part 2

fn part2() -> u64 {
  iter(256)
}

#[test]
fn part2_test() {
  assert_eq!(part2(), 1595779846729);
}

// main

fn main() {
  println!("--- day 1 ---");
  println!("part 1: {}", part1());
  println!("part 2: {}", part2());
}
