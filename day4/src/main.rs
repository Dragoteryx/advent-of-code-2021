// bingo

#[derive(Debug, Clone)]
struct Bingo([[(u16, bool); 5]; 5]);

impl Bingo {
  pub fn new(numbers: [[u16; 5]; 5]) -> Self {
    let mut bingo = Self([[(0, false); 5]; 5]);
    for (i, line) in numbers.iter().enumerate() {
      for (j, num) in line.iter().enumerate() {
        bingo.0[i][j] = (*num, false);
      }
    }
    bingo
  }

  pub fn line(&self, line: usize) -> [(u16, bool); 5] {
    self.0[line]
  }

  pub fn lines(&self) -> impl Iterator<Item = [(u16, bool); 5]> + '_ {
    (0..5).map(|i| self.line(i))
  }

  pub fn column(&self, col: usize) -> [(u16, bool); 5] {
    let mut column = [(0, false); 5];
    for (i, line) in self.0.iter().enumerate() {
      column[i] = line[col];
    }
    column
  }

  pub fn columns(&self) -> impl Iterator<Item = [(u16, bool); 5]> + '_ {
    (0..5).map(|i| self.column(i))
  }

  pub fn lines_columns(&self) -> impl Iterator<Item = [(u16, bool); 5]> + '_ {
    self.lines().chain(self.columns())
  }

  pub fn winning(&self) -> bool {
    'outer: for values in self.lines_columns() {
      for value in values {
        if !value.1 {
          continue 'outer;
        }
      }
      return true;
    }
    false
  }

  pub fn score(&self, value: u16) -> u16 {
    let sum = self.lines()
      .flatten()
      .filter(|(_, ok)| !ok)
      .map(|(num, _)| num)
      .sum::<u16>();
     sum * value 
  }

  pub fn drawn(&mut self, value: u16) {
    for line in self.0.iter_mut() {
      for num in line.iter_mut() {
        if num.0 == value {
          num.1 = true;
        }
      }
    }
  }
}

// util

fn read_input() -> (Vec<u16>, Vec<Bingo>) {
  let str = include_str!("input.txt");
  let mut lines = str.lines();
  let numbers = lines.next().unwrap()
    .split(',')
    .map(|s| s.parse().unwrap())
    .collect();
  let mut bingos = Vec::new();
  while lines.next().is_some() {
    let mut bingo = [[0; 5]; 5];
    for bingo_line in bingo.iter_mut() {
      let mut line = lines.next().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap());
      for bingo_num in bingo_line.iter_mut() {
        *bingo_num = line.next().unwrap();
      }
    }
    bingos.push(Bingo::new(bingo));
  }
  (numbers, bingos)
}

// part 1

fn part1() -> u16 {
  let (numbers, mut bingos) = read_input();
  for num in numbers {
    for bingo in bingos.iter_mut() {
      bingo.drawn(num);
      if bingo.winning() {
        return bingo.score(num);
      }
    }
  }
  panic!("No winning bingo found");
}

#[test]
fn part1_test() {
  assert_eq!(part1(), 58374);
}

// part 2

fn part2() -> u16 {
  let (numbers, mut bingos) = read_input();
  let bingos_len = bingos.len();
  let mut won = 0;
  for num in numbers {
    for bingo in bingos.iter_mut() {
      if bingo.winning() { continue; }
      bingo.drawn(num);
      if bingo.winning() {
        won += 1;
        if won == bingos_len {
          return bingo.score(num);
        }
      }
    }
  }
  panic!("No winning bingo found");
}

#[test]
fn part2_test() {
  assert_eq!(part2(), 11377);
}

// main

fn main() {
  println!("--- day 4 ---");
  println!("part 1: {}", part1());
  println!("part 2: {}", part2());
}