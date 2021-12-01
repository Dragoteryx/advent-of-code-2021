use std::io::{stdin, stdout, Write};
use std::str::FromStr;

// days

mod day1;

// main

const CALENDAR: [[fn(); 2]; 1] = [
  [day1::part1, day1::part2]
];

fn print(msg: &str) {
  print!("{}", msg);
  stdout().flush().unwrap();
}

fn input() -> String {
  let mut input = String::new();
  stdin().read_line(&mut input).unwrap();
  input.trim().into()
}

fn main() {
  print("Day: ");
  let day = input();
  if let Ok(day) = usize::from_str(&day) {
    if let Some(day) = CALENDAR.get(day - 1) {
      print("Part: ");
      let part = input();
      if let Ok(part) = usize::from_str(&part) {
        if let Some(part) = day.get(part - 1) {
          part();
        } else {
          println!("Part {} not found", part);
        }
      } else {
        println!("Invalid part: {}", part);
      }
    } else {
      println!("Day {} not found", day);
    }
  } else {
    println!("Invalid day: {}", day);
  }
}