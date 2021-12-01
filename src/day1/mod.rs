use std::str::FromStr;
use std::fs;

// util

fn depths() -> Vec<u16> {
  fs::read_to_string("src/day1/data.txt")
    .expect("Failed to read data")
    .lines()
    .map(|depth| u16::from_str(depth).expect("Failed to parse depth"))
    .collect()
}

// part 1

pub fn part1() {
  let depths = depths();
  let mut increased = 0u16;
  for (i, depth) in depths.iter().enumerate() {
    if let Some(previous) = depths.get(i - 1) {
      if depth > previous {
        increased += 1;
      }
    }
  }
  println!("The depth increased {} times", increased);
}

// part 2

pub fn part2() {
  let depths = depths();
  let mut increased = 0u16;
  for (i, depth_a) in depths.iter().enumerate() {
    if let (Some(depth_z), Some(depth_b), Some(depth_c)) = (
      depths.get(i - 1),
      depths.get(i + 1),
      depths.get(i + 2)
    ) {
      let previous = depth_z + depth_a + depth_b;
      let measurement = depth_a + depth_b + depth_c;
      if measurement > previous {
        increased += 1;
      }
    }
  }
  println!("The measurement increased {} times", increased);
}