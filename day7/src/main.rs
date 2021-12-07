use std::cmp::Ordering;
use std::mem::swap;

// util

fn positions() -> Vec<u32> {
  include_str!("input.txt")
    .split(',')
    .map(|s| s.parse().unwrap())
    .collect()
}

fn calc_fuel(calc: impl Fn(u32, u32) -> u32) -> (u32, u32) {
  let positions = positions();
  let min = *positions.iter().min().unwrap();
  let max = *positions.iter().max().unwrap();
  (min..=max).into_iter()
    .map(|pos| (pos, positions.iter().fold(0, |acc, &p| acc + calc(p, pos))))
    .reduce(|acc, (pos, fuel)| {
      if fuel < acc.1 {
        (pos, fuel)
      } else {
        acc
      }
    }).unwrap()
}

// part 1

fn part1() -> (u32, u32) {
  calc_fuel(|from, to| match from.cmp(&to) {
    Ordering::Less => to - from,
    Ordering::Greater => from - to,
    Ordering::Equal => 0
  })
}

#[test]
fn part1_test() {
  assert_eq!(part1(), (362, 342534));
}

// part 2

fn part2() -> (u32, u32) {
  calc_fuel(|mut from, mut to| {
    if from == to { return 0; }
    if from > to { swap(&mut from, &mut to); }
    (1..=(to - from)).into_iter().sum()
  })
}

#[test]
fn part2_test() {
  assert_eq!(part2(), (474, 94004208));
}

// main

fn main() {
  println!("--- day 7 ---");
  let (pos, fuel) = part1();
  println!("part 1: [pos: {}, fuel: {}]", pos, fuel);
  let (pos, fuel) = part2();
  println!("part 2: [pos: {}, fuel: {}]", pos, fuel);
}