#[aoc(day1, part1)]
pub fn solve_part1(data: &str) -> i32 {
  let mut most_cal: i32 = 0;
  let mut count: i32 = 0;

  for line in data.lines() {
    if line == "" {
      count = 0;
    }

    if line != "" {
      let int: i32 = line.parse().unwrap();

      count = count + int;

      if count > most_cal {
        most_cal = count;
      }
    }
  }

  most_cal
}

#[aoc(day1, part2)]
pub fn solve_part2(data: &str) -> i32 {
  let mut list_totals: Vec<i32> = Vec::new();
  let mut most_cal: i32 = 0;
  let mut count: i32 = 0;
  let mut answer: i32 = 0;

  for line in data.lines() {
    if line == "" {
      // check if stack contains the value if not add it
      if !list_totals.contains(&most_cal) { list_totals.push(most_cal) }
      count = 0;
    
      // hack
      if list_totals.len() > 3 {
        most_cal = list_totals[2];
      }
    }

    if line != "" {
      let int: i32 = line.parse().unwrap();

      count = count + int;

      if count > most_cal {
        most_cal = count;
      }
    }
  }

  // pop values from stack but only add up last 3
  let mut itr: i32 = 0;
  while let Some(top) = list_totals.pop() {
    if itr < 3 { answer += top; }
    itr += 1;
  }

  answer
}