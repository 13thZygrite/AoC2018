use std::collections::HashMap;

#[aoc_generator(day1)]
fn day1_input(input: &str) -> Vec<i32> {
  input.lines().map(|s| s.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
fn day1_part1(changes: &Vec<i32>) -> i32 {
  changes.iter().sum()
}

#[aoc(day1, part2)]
fn day1_part2(changes: &Vec<i32>) -> i32 {
  let mut freq = 0;
  let mut seen = HashMap::new();
  let endless = changes.iter().cycle();
  for value in endless {
    freq += value;
    if seen.contains_key(&freq) {
      break;
    } else {
      seen.insert(freq, freq);
    }
  }

  freq
}



