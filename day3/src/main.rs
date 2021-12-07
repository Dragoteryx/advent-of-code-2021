use std::cmp::Ordering;

// util

const N: usize = 12;
type Bits = [bool; N];

fn numbers() -> Vec<Bits> {
  include_str!("input.txt")
    .lines()
    .map(|number| {
      let mut bits = [false; N];
      for (i, bit) in number.chars().enumerate() {
        bits[i] = match bit {
          '0' => false,
          '1' => true,
          _ => panic!("Expected 0 or 1, got {}", bit)
        };
      }
      bits
    }).collect()
}

fn bits_to_number(bits: Bits) -> u32 {
  let mut num = 0;
  for (bit, n) in bits.into_iter().zip((0..N).rev()) {
    if bit { num += 1 << n }
  }
  num
}

// part 1

fn most_common_bit(bit: usize, numbers: &[Bits]) -> Option<bool> {
  let mut one: u32 = 0;
  let mut zero: u32 = 0;
  for number in numbers {
    match number[bit] {
      true => one += 1,
      false => zero += 1
    }
  }
  match one.cmp(&zero) {
    Ordering::Greater => Some(true),
    Ordering::Less => Some(false),
    Ordering::Equal => None
  }
}

fn least_common_bit(bit: usize, numbers: &[Bits]) -> Option<bool> {
  most_common_bit(bit, numbers).map(|bit| !bit)
}

fn gamma_rate(numbers: &[Bits]) -> u32 {
  let mut rate = 0;
  for (i, n) in (0..N).zip((0..N).rev()) {
    if most_common_bit(i, numbers).unwrap_or(true) {
      rate += 1 << n;
    }
  }
  rate
}

fn epsilon_rate(numbers: &[Bits]) -> u32 {
  let mut rate = 0;
  for (i, n) in (0..N).zip((0..N).rev()) {
    if least_common_bit(i, numbers).unwrap_or(true) {
      rate += 1 << n;
    }
  }
  rate
}

fn part1() -> u32 {
  let numbers = numbers();
  gamma_rate(&numbers) * epsilon_rate(&numbers)
}

#[test]
fn part1_test() {
  assert_eq!(part1(), 3901196);
}

// part 2

fn find_by_criteria(numbers: &[Bits], bit: usize, criteria: impl Fn(usize, &[Bits]) -> bool) -> u32 {
  let crit = criteria(bit, numbers);
  let filtered: Vec<_> = numbers.iter()
    .filter(|bits| bits[bit] == crit)
    .copied().collect();
  match filtered[..] {
    [bits] => bits_to_number(bits),
    _ => find_by_criteria(&filtered, bit+1, criteria)
  }
}

fn oxygen_generator_rating(numbers: &[Bits]) -> u32 {
  find_by_criteria(numbers, 0, |bit, numbers| most_common_bit(bit, numbers).unwrap_or(true))
}

fn co2_scrubber_rating(numbers: &[Bits]) -> u32 {
  find_by_criteria(numbers, 0, |bit, numbers| least_common_bit(bit, numbers).unwrap_or(false))
}

fn part2() -> u32 {
  let numbers = numbers();
  oxygen_generator_rating(&numbers) * co2_scrubber_rating(&numbers)
}

#[test]
fn part2_test() {
  assert_eq!(part2(), 4412188);
}

// main

fn main() {
  println!("--- day 3 ---");
  println!("part 1: {}", part1());
  println!("part 2: {}", part2());
}