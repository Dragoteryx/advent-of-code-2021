use std::str::FromStr;
use std::fs;

// part 1

pub fn part1() {
  let depths = fs::read_to_string("src/day1/data.txt").expect("Failed to read data");
  let mut increased = 0u16;
  let mut previous: Option<u16> = None;
  for depth in depths.lines() {
    let depth = u16::from_str(depth).expect("Failed to parse depth");
    if let Some(previous) = previous {
      if depth > previous {
        increased += 1;
      }
    }
    previous = Some(depth);
  }
  println!("The depth increased {} times", increased);
}

// part 2

pub struct Measurements<T: Iterator<Item=u16>> {
  depths: T, current: Option<(u16, u16, u16)>
}

impl<T: Iterator<Item=u16>> Measurements<T> {
  pub fn new(depths: T) -> Self {
    Measurements { depths, current: None }
  }
}

impl<T: Iterator<Item=u16>> Iterator for Measurements<T> {
  type Item = u16;

  fn next(&mut self) -> Option<Self::Item> {
    match self.current {
      None => {
        let a = self.depths.next().expect("Failed to get first depth");
        let b = self.depths.next().expect("Failed to get second depth");
        let c = self.depths.next().expect("Failed to get third depth");
        self.current = Some((a, b, c));
        Some(a + b + c)
      }
      Some((_, a, b)) => {
        if let Some(c) = self.depths.next() {
          self.current = Some((a, b, c));
          Some(a + b + c)
        } else {
          None
        }
      }
    }
  }
}

pub fn part2() {
  let depths = fs::read_to_string("src/day1/data.txt").expect("Failed to read data");
  let measurements = Measurements::new(depths.lines().map(|depth| u16::from_str(depth).expect("Failed to parse depth")));
  let mut increased = 0u16;
  let mut previous: Option<u16> = None;
  for measurement in measurements {
    if let Some(previous) = previous {
      if measurement > previous {
        increased += 1;
      }
    }
    previous = Some(measurement);
  }
  println!("The measurement increased {} times", increased);
}