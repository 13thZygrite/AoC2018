pub struct Region {
  num: u32,
  x: usize,
  y: usize,
  w: usize,
  h: usize 
}


#[aoc_generator(day3)]
fn day3_input(input: &str) -> Vec<Region> {
  input.lines().map(|l| {
    let mut parts = l.split(|s| s == '@' || s == ',' || s == ':' || s == 'x');
    let num = parts.next().unwrap()[1..].trim().parse().unwrap();
    let x = parts.next().unwrap().trim().parse().unwrap();
    let y = parts.next().unwrap().trim().parse().unwrap();
    let w = parts.next().unwrap().trim().parse().unwrap();
    let h = parts.next().unwrap().trim().parse().unwrap();
    Region { num, x, y, w, h}    
  }).collect()
   
}

#[aoc(day3, part1)]
fn day3_part1(input: &Vec<Region>) -> u32 {
   let mut cloth = vec![vec![0; 1000]; 1000];
   for region in input {
     for i in region.x..region.x+region.w {
       for j in region.y..region.y+region.h {
         cloth[i][j] += 1;
       }
     }
   }
  let mut count = 0;
  for i in 0..1000 {
    for j in 0..1000 {
      count += if cloth[i][j] > 1 { 1 } else {0};
    }
  }
  count
}

#[aoc(day3, part2)]
fn day3_part2(input: &Vec<Region>) -> u32 {
   let mut cloth = vec![vec![0; 1000]; 1000];
   for region in input.clone() {
     for i in region.x..region.x+region.w {
       for j in region.y..region.y+region.h {
         cloth[i][j] += 1;
       }
     }
   }
  let mut answer = 0;
  for region in input {
     let mut fine = true;
     for i in region.x..region.x+region.w {
       for j in region.y..region.y+region.h {
         if cloth[i][j] != 1 {fine = false};
       }
     }
    if fine {answer = region.num};
   }
  answer
}
