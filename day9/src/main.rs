#![allow(clippy::ptr_arg)]

use std::collections::HashSet;
use itertools::Itertools;

// util

type Heightmap = Vec<Vec<u16>>;
type Basin<'a> = HashSet<Location<'a>>;

fn heightmap() -> Heightmap {
  include_str!("input.txt")
    .lines()
    .map(|line| {
      line.chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect()
    })
    .collect()
}

fn locations(heightmap: &Heightmap) -> impl Iterator<Item = Location> + '_ {
  heightmap.iter().enumerate().flat_map(move |(y, row)| {
    row.iter().enumerate().map(move |(x, _)| Location::new(heightmap, x, y))
  })
}

// location

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Location<'a> {
  heightmap: &'a Heightmap,
  x: usize, y: usize
}

impl<'a> Location<'a> {
  pub fn new(heightmap: &'a Heightmap, x: usize, y: usize) -> Self {
    Self { heightmap, x, y }
  }

  pub fn height(&self) -> u16 {
    self.heightmap[self.y][self.x]
  }

  pub fn risk(&self) -> u16 {
    self.height() + 1
  }

  pub fn top(&self) -> Option<Self> {
    if self.y > 0 {
      Some(Self::new(self.heightmap, self.x, self.y - 1))
    } else {
      None
    }
  }

  pub fn bottom(&self) -> Option<Self> {
    if self.y < self.heightmap.len() - 1 {
      Some(Self::new(self.heightmap, self.x, self.y + 1))
    } else {
      None
    }
  }

  pub fn left(&self) -> Option<Self> {
    if self.x > 0 {
      Some(Self::new(self.heightmap, self.x - 1, self.y))
    } else {
      None
    }
  }

  pub fn right(&self) -> Option<Self> {
    if self.x < self.heightmap[0].len() - 1 {
      Some(Self::new(self.heightmap, self.x + 1, self.y))
    } else {
      None
    }
  }

  pub fn is_low_point(&self) -> bool {
    if let Some(top) = self.top() {
      if self.height() >= top.height() { return false; }
    }
    if let Some(bottom) = self.bottom() {
      if self.height() >= bottom.height() { return false; }
    }
    if let Some(left) = self.left() {
      if self.height() >= left.height() { return false; }
    }
    if let Some(right) = self.right() {
      if self.height() >= right.height() { return false; }
    }
    true
  }

  pub fn basin(&self) -> Basin<'a> {
    if self.height() == 9 {
      Basin::new()
    } else {
      let mut basin = Basin::new();
      basin.insert(self.clone());
      if let Some(top) = self.top() {
        if self.height() < top.height() {
          basin = &basin | &top.basin();
        }
      }
      if let Some(bottom) = self.bottom() {
        if self.height() < bottom.height() {
          basin = &basin | &bottom.basin();
        }
      }
      if let Some(left) = self.left() {
        if self.height() < left.height() {
          basin = &basin | &left.basin();
        }
      }
      if let Some(right) = self.right() {
        if self.height() < right.height() {
          basin = &basin | &right.basin();
        }
      }
      basin
    }
  }

  pub fn basin_size(&self) -> usize {
    self.basin().len()
  }
}

// part 1

fn part1() -> u16 {
  locations(&heightmap())
    .filter(|loc| loc.is_low_point())
    .map(|loc| loc.risk())
    .sum()
}

#[test]
fn part1_test() {
  assert_eq!(part1(), 562);
}

// part 2

fn part2() -> usize {
  locations(&heightmap())
    .filter(|loc| loc.is_low_point())
    .map(|loc| loc.basin_size())
    .sorted()
    .rev()
    .take(3)
    .product()
}

#[test]
fn part2_test() {
  assert_eq!(part2(), 1076922);
}

// main

fn main() {
  println!("--- day 9 ---");
  println!("part 1: {}", part1());
  println!("part 2: {}", part2());
}
