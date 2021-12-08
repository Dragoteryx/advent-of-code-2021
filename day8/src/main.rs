use std::ops::{Deref, Add, Sub};
use std::collections::HashSet;
use itertools::Itertools;

// signal

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Signal(String);

impl Signal {
  pub fn new(s: &str) -> Self {
    Self(s.chars().sorted().collect())
  }

  pub fn is_one(&self) -> bool {
    self.len() == 2
  }

  pub fn is_four(&self) -> bool {
    self.len() == 4
  }

  pub fn is_seven(&self) -> bool {
    self.len() == 3
  }

  pub fn is_eight(&self) -> bool {
    self.len() == 7
  }
}

impl Deref for Signal {
  type Target = String;

  fn deref(&self) -> &String {
    &self.0
  }
}

impl Add for &Signal {
  type Output = Signal;

  fn add(self, other: Self) -> Signal {
    let mut chars = HashSet::new();
    for c in self.chars() { chars.insert(c); }
    for c in other.chars() { chars.insert(c); }
    let str: String = chars.into_iter().collect();
    Signal::new(&str)
  }
}

impl Sub for &Signal {
  type Output = Signal;

  fn sub(self, other: Self) -> Signal {
    let mut chars = HashSet::new();
    for c in self.chars() { chars.insert(c); }
    for c in other.chars() { chars.remove(&c); }
    let s: String = chars.into_iter().collect();
    Signal::new(&s)
  }
}

// util

struct Entry {
  pub inputs: [Signal; 10],
  pub outputs: [Signal; 4]
}

impl Entry {
  pub fn iter() -> impl Iterator<Item = Entry> {
    include_str!("input.txt")
    .lines()
    .map(|line| {
      let mut line = line.split_whitespace();
      let inputs = [
        Signal::new(line.next().unwrap()),
        Signal::new(line.next().unwrap()),
        Signal::new(line.next().unwrap()),
        Signal::new(line.next().unwrap()),
        Signal::new(line.next().unwrap()),
        Signal::new(line.next().unwrap()),
        Signal::new(line.next().unwrap()),
        Signal::new(line.next().unwrap()),
        Signal::new(line.next().unwrap()),
        Signal::new(line.next().unwrap())
      ];
      line.next().unwrap();
      let outputs = [
        Signal::new(line.next().unwrap()),
        Signal::new(line.next().unwrap()),
        Signal::new(line.next().unwrap()),
        Signal::new(line.next().unwrap())
      ];
      Entry { outputs, inputs }
    })
  }
}

// part 1

fn part1() -> usize {
  Entry::iter()
    .map(|entry| entry.outputs)
    .flatten()
    .filter(|sig| matches!(sig.len(), 2 | 3 | 4 | 7))
    .count()
}

// part 2

fn part2() -> u32 {
  Entry::iter().map(|entry| {
    let one = entry.inputs.iter().find(|sig| sig.is_one()).unwrap();
    let four = entry.inputs.iter().find(|sig| sig.is_four()).unwrap();
    let seven = entry.inputs.iter().find(|sig| sig.is_seven()).unwrap();
    let eight = entry.inputs.iter().find(|sig| sig.is_eight()).unwrap();

    let len5 = || entry.inputs.iter().filter(|sig| sig.len() == 5);
    let two = len5().find(|sig| (*sig - four).len() == 3).unwrap();
    let three = len5().find(|sig| (*sig - one).len() == 3).unwrap();
    let five = len5().find(|sig| *sig != two && *sig != three).unwrap();

    let len6 = || entry.inputs.iter().filter(|sig| sig.len() == 6);
    let six = len6().find(|sig| (*sig - one).len() == 5).unwrap();
    let nine = len6().find(|sig| (*sig - &(four + seven)).len() == 1).unwrap();
    let _zero = len6().find(|sig| *sig != six && *sig != nine).unwrap();

    let mut value: u32 = 0;
    let [thousands, hundreds, tens, unit] = entry.outputs;

    if &thousands == one { value += 1000; }
    else if &thousands == two { value += 2000; }
    else if &thousands == three { value += 3000; }
    else if &thousands == four { value += 4000; }
    else if &thousands == five { value += 5000; }
    else if &thousands == six { value += 6000; }
    else if &thousands == seven { value += 7000; }
    else if &thousands == eight { value += 8000; }
    else if &thousands == nine { value += 9000; }

    if &hundreds == one { value += 100; }
    else if &hundreds == two { value += 200; }
    else if &hundreds == three { value += 300; }
    else if &hundreds == four { value += 400; }
    else if &hundreds == five { value += 500; }
    else if &hundreds == six { value += 600; }
    else if &hundreds == seven { value += 700; }
    else if &hundreds == eight { value += 800; }
    else if &hundreds == nine { value += 900; }

    if &tens == one { value += 10; }
    else if &tens == two { value += 20; }
    else if &tens == three { value += 30; }
    else if &tens == four { value += 40; }
    else if &tens == five { value += 50; }
    else if &tens == six { value += 60; }
    else if &tens == seven { value += 70; }
    else if &tens == eight { value += 80; }
    else if &tens == nine { value += 90; }

    if &unit == one { value += 1; }
    else if &unit == two { value += 2; }
    else if &unit == three { value += 3; }
    else if &unit == four { value += 4; }
    else if &unit == five { value += 5; }
    else if &unit == six { value += 6; }
    else if &unit == seven { value += 7; }
    else if &unit == eight { value += 8; }
    else if &unit == nine { value += 9; }

    value
  }).sum()
}

// main

fn main() {
  println!("--- day 8 ---");
  println!("part 1: {}", part1());
  println!("part 2: {}", part2());
}