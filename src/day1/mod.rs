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
  println!("The depth increased {} times", part1_main());
}

fn part1_main() -> u16 {
  let depths = depths();
  let mut increased = 0u16;
  for (i, depth) in depths.iter().enumerate() {
    if let Some(next) = depths.get(i + 1) {
      if depth < next {
        increased += 1;
      }
    }
  }
  increased
}

#[cfg(test)] #[test]
fn part1_test() {
  assert_eq!(part1_main(), 1692);
}

// part 2

pub fn part2() {
  println!("The measurement increased {} times", part2_main());
}

fn part2_main() -> u16 {
  let depths = depths();
  let mut increased = 0u16;
  for (i, depth_a) in depths.iter().enumerate() {
    if let (Some(depth_b), Some(depth_c), Some(depth_d)) = (
      depths.get(i + 1),
      depths.get(i + 2),
      depths.get(i + 3)
    ) {
      let measurement = depth_a + depth_b + depth_c;
      let next = depth_b + depth_c + depth_d;
      if measurement < next {
        increased += 1;
      }
    }
  }
  increased
}

#[cfg(test)] #[test]
fn part2_test() {
  assert_eq!(part2_main(), 1724);
}