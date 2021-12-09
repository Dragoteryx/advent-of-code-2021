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
  heightmap.iter()
    .enumerate()
    .flat_map(move |(y, row)| {
      row.iter()
        .enumerate()
        .map(move |(x, _)| Location::new(heightmap, x, y))
    })
}

fn low_points(heightmap: &Heightmap) -> impl Iterator<Item = Location> + '_ {
  locations(heightmap)
    .filter(|loc| loc.is_low_point())
}

// location

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

  pub fn adjacent(&self) -> impl Iterator<Item = Location<'a>> + '_ {
    [self.top(), self.bottom(), self.left(), self.right()]
      .into_iter()
      .flatten()
  }

  pub fn is_low_point(&self) -> bool {
    self.adjacent().all(|loc| loc.height() > self.height())
  }

  pub fn basin(&self) -> Option<Basin<'a>> {
    if self.height() == 9 {
      None
    } else {
      let mut basin = Basin::from([*self]);
      self.adjacent()
        .filter(|loc| loc.height() > self.height())
        .flat_map(|loc| loc.basin())
        .for_each(|basin2| basin.extend(basin2));
      Some(basin)
    }
  }

  pub fn basin_size(&self) -> usize {
    self.basin().map(|basin| basin.len()).unwrap_or(0)
  }
}

// part 1

fn part1() -> u16 {
  low_points(&heightmap())
    .map(|loc| loc.risk())
    .sum()
}

#[test]
fn part1_test() {
  assert_eq!(part1(), 562);
}

// part 2

fn part2() -> usize {
  low_points(&heightmap())
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
