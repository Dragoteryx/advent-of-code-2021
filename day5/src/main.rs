use std::collections::HashMap;
use std::cmp::Ordering;

// vent

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vents {
  from: (u32, u32),
  to: (u32, u32),
}

impl Vents {
  pub fn iter(&self, diagonals: bool) -> VentsIter {
    let Self { from, to } = *self;
    VentsIter {
      from: Some(from),
      to, diagonals
    }
  }
}

// vents line

struct VentsIter {
  from: Option<(u32, u32)>,
  to: (u32, u32),
  diagonals: bool
}

impl Iterator for VentsIter {
  type Item = (u32, u32);

  fn next(&mut self) -> Option<Self::Item> {
    if let Some((x1, y1)) = self.from {
      let (x2, y2) = self.to;
      if x1 == x2 {
        self.from = match y1.cmp(&y2) {
          Ordering::Less => Some((x1, y1 + 1)),
          Ordering::Greater => Some((x1, y1 - 1)),
          Ordering::Equal => None
        };
        Some((x1, y1))
      } else if y1 == y2 {
        self.from = match x1.cmp(&x2) {
          Ordering::Less => Some((x1 + 1, y1)),
          Ordering::Greater => Some((x1 - 1, y1)),
          Ordering::Equal => None
        };
        Some((x1, y1))
      } else if self.diagonals {
        self.from = match (x1.cmp(&x2), y1.cmp(&y2)) {
          (Ordering::Less, Ordering::Less) => Some((x1 + 1, y1 + 1)),
          (Ordering::Less, Ordering::Greater) => Some((x1 + 1, y1 - 1)),
          (Ordering::Greater, Ordering::Less) => Some((x1 - 1, y1 + 1)),
          (Ordering::Greater, Ordering::Greater) => Some((x1 - 1, y1 - 1)),
          _ => None
        };
        Some((x1, y1))
      } else {
        None
      }
    } else {
      None
    }
  }
}

// util

fn vents() -> impl Iterator<Item = Vents> {
  include_str!("input.txt")
    .lines()
    .map(|line| {
      let mut iter = line.split(" -> ");
      let mut from = iter.next().unwrap().split(',').map(|x| x.parse().unwrap());
      let mut to = iter.next().unwrap().split(',').map(|x| x.parse().unwrap());
      let (x1, y1) = (from.next().unwrap(), from.next().unwrap());
      let (x2, y2) = (to.next().unwrap(), to.next().unwrap());
      Vents { from: (x1, y1), to: (x2, y2) }
    })
}

fn calculate_overlaps(diagonals: bool) -> u32 {
  let mut grid = HashMap::new();
  for vent in vents() {
    for point in vent.iter(diagonals) {
      *grid.entry(point).or_insert(0u32) += 1;
    }
  }
  let mut overlaps = 0;
  for (_, value) in grid {
    if value > 1 {
      overlaps += 1;
    }
  }
  overlaps
}

// part 1

fn part1() -> u32 {
  calculate_overlaps(false)
}

#[test]
fn part1_test() {
  assert_eq!(part1(), 8060);
}

// part 2

fn part2() -> u32 {
  calculate_overlaps(true)
}

#[test]
fn part2_test() {
  assert_eq!(part2(), 21577);
}

// main

fn main() {
  println!("--- day 5 ---");
  println!("part 1: {}", part1());
  println!("part 2: {}", part2());
}