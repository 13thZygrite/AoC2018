#[aoc_generator(day2)]
fn day2_input(input: &str) -> Vec<Vec<char>> {
  input.lines().map(|s| s.chars().collect::<Vec::<char>>()).collect()
}

#[aoc(day2, part1)]
fn day2_part1(input: &Vec<Vec<char>>) -> u32 {
  let mut count2 = 0;
  let mut count3 = 0;
  for mut line in input.clone() {
    line.sort();
    for i in 0..line.len()-2 {
        if i == 0 && line[i] == line[i+1] && line[i] != line[i+2] { count2 += 1; break}
        if i != 0 && line[i] != line[i-1] &&line[i] == line[i+1] && line[i] != line[i+2] { count2 += 1; break}
        if i == line.len() - 3 && line[i+1] == line[i+2] && line[i] != line[i+1] { count2 += 1; break}
    }
    for i in 0..line.len()-3 {
        if i == 0 && line[i] == line[i+1] && line[i+1] == line[i+2] && line[i] != line[i+3] { count3 += 1; break}
        if i != 0 && line[i] != line[i-1] && line[i] == line[i+1] && line[i+1] == line[i+2] && line[i] != line[i+3] { count3 += 1; break}
        if i == line.len() - 4 && line[i+1] == line[i+2] && line[i+2] == line[i+3] && line[i] != line[i+1] { count3 += 1; break}
    }
  }
  println!("{}, {}", count2, count3);
  count2 * count3

}

#[aoc(day2, part2)]
fn day2_part2(input: &Vec<Vec<char>>) -> u32 {
  for i in 0..input.len()-1 {
    for j in i+1..input.len() {
      let mut count = 0;
      for k in 0..input[i].len() {
        if input[i][k] != input[j][k] { count += 1}
      }
      if count == 1 {
        let s1: String = input[i].iter().collect();
        let s2: String = input[j].iter().collect();
        println!("{}", s1);
        println!("{}", s2);
      }
    }
  }
 1
}
