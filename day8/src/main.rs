use std::collections::HashSet;

// char 

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Char {
  A, B, C, D,
  E, F, G
}

impl From<char> for Char {
  fn from(c: char) -> Self {
    match c {
      'a' => Self::A,
      'b' => Self::B,
      'c' => Self::C,
      'd' => Self::D,
      'e' => Self::E,
      'f' => Self::F,
      'g' => Self::G,
      _ => panic!("Invalid char")
    }
  }
}

// signal

type Signal = HashSet<Char>;
fn signal(s: &str) -> Signal {
  s.chars().map(Char::from).collect()
}

// entry

#[derive(Debug, Clone)]
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
        signal(line.next().unwrap()),
        signal(line.next().unwrap()),
        signal(line.next().unwrap()),
        signal(line.next().unwrap()),
        signal(line.next().unwrap()),
        signal(line.next().unwrap()),
        signal(line.next().unwrap()),
        signal(line.next().unwrap()),
        signal(line.next().unwrap()),
        signal(line.next().unwrap())
      ];
      line.next().unwrap();
      let outputs = [
        signal(line.next().unwrap()),
        signal(line.next().unwrap()),
        signal(line.next().unwrap()),
        signal(line.next().unwrap())
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

#[test]
fn part1_test() {
  assert_eq!(part1(), 381);
}

// part 2

fn part2() -> u32 {
  Entry::iter().map(|entry| {
    let one = entry.inputs.iter().find(|sig| sig.len() == 2).unwrap();
    let four = entry.inputs.iter().find(|sig| sig.len() == 4).unwrap();
    let seven = entry.inputs.iter().find(|sig| sig.len() == 3).unwrap();
    let eight = entry.inputs.iter().find(|sig| sig.len() == 7).unwrap();
    let four_diff_one = &(four - one);

    let len5 = || entry.inputs.iter().filter(|sig| sig.len() == 5);
    let three = len5().find(|sig| sig.is_superset(one)).unwrap();
    let five = len5().find(|sig| sig.is_superset(four_diff_one)).unwrap();
    let two = len5().find(|sig| *sig != three && *sig != five).unwrap();

    let len6 = || entry.inputs.iter().filter(|sig| sig.len() == 6);
    let nine = len6().find(|sig| sig.is_superset(four)).unwrap();
    let six = len6().find(|sig| *sig != nine && sig.is_superset(four_diff_one)).unwrap();
    let _zero = len6().find(|sig| *sig != nine && *sig != six).unwrap();

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

#[test]
fn part2_test() {
  assert_eq!(part2(), 1023686);
}

// main

fn main() {
  println!("--- day 8 ---");
  println!("part 1: {}", part1());
  println!("part 2: {}", part2());
}