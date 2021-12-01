use std::str::FromStr;
use std::fs;

pub fn run() {
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