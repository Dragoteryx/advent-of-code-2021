use std::fs;

// part 1

fn depths() -> Vec<u16> {
  fs::read_to_string("src/input.txt")
    .expect("Failed to read input")
    .lines()
    .map(|depth| depth.parse().expect("Failed to parse depth"))
    .collect()
}

fn part1() -> u16 {
  let depths = depths();
  let mut increased = 0;
  for window in depths.windows(2) {
    let (depth1, depth2) = (window[0], window[1]);
    if depth1 < depth2 { increased += 1 }
  }
  increased
}

#[test]
fn part1_test() {
  assert_eq!(part1(), 1692);
}

// part 2

fn measurements() -> Vec<[u16; 3]> {
  depths().windows(3)
    .map(|w| [w[0], w[1], w[2]])
    .collect()
}

fn part2() -> u16 {
  let measurements = measurements();
  let mut increased = 0;
  for w in measurements.windows(2) {
    let (m1, m2) = (w[0], w[1]);
    let depth1 = m1[0] + m1[1] + m1[2];
    let depth2 = m2[0] + m2[1] + m2[2];
    if depth1 < depth2 { increased += 1 }
  }
  increased
}

#[test]
fn part2_test() {
  assert_eq!(part2(), 1724);
}

// main

fn main() {
  println!("part 1: {}", part1());
  println!("part 2: {}", part2());
}