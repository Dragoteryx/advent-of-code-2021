// util

fn positions() -> Vec<i32> {
  include_str!("input.txt")
    .split(',')
    .map(|s| s.parse().unwrap())
    .collect()
}

fn calc_fuel(fuel: impl Fn(i32, i32) -> i32) -> (i32, i32) {
  let positions = positions();
  let min = *positions.iter().min().unwrap();
  let max = *positions.iter().max().unwrap();
  (min..=max).into_iter()
    .map(|pos| (pos, positions.iter().fold(0, |acc, &p| acc + fuel(p, pos))))
    .reduce(|(pos, fuel), (new_pos, new_fuel)| {
      if new_fuel < fuel {
        (new_pos, new_fuel)
      } else {
        (pos, fuel)
      }
    }).unwrap()
}

// part 1

fn part1() -> (i32, i32) {
  calc_fuel(|from, to| (from - to).abs())
}

#[test]
fn part1_test() {
  assert_eq!(part1(), (362, 342534));
}

// part 2

fn part2() -> (i32, i32) {
  calc_fuel(|from, to| {
    let n = (from - to).abs();
    n * (n+1) / 2
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