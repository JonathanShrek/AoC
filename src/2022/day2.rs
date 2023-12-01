#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> i32 {
  const LOSE_POINTS: i32 = 0;
  const DRAW_POINTS: i32 = 3;
  const WIN_POINTS: i32 = 6;

  const ROCK_POINTS: i32 = 1;
  const PAPER_POINTS: i32 = 2;
  const SCISSOR_POINTS: i32 = 3;

  let mut total_score: i32 = 0;
  let mut round_score: i32 = 0;

  for line in input.lines() {
    match line {
      // rock vs paper
      "A Y" => { round_score = PAPER_POINTS + WIN_POINTS; },
      // rock vs rock
      "A X" => { round_score = ROCK_POINTS + DRAW_POINTS; },
      // rock vs scissors
      "A Z" => { round_score = SCISSOR_POINTS + LOSE_POINTS; },
      // paper vs paper
      "B Y" => { round_score = PAPER_POINTS + DRAW_POINTS; },
      // paper vs rock
      "B X" => { round_score = ROCK_POINTS + LOSE_POINTS; },
      // paper vs scissors
      "B Z" => { round_score = SCISSOR_POINTS + WIN_POINTS; },
      // scissors vs paper
      "C Y" => { round_score = PAPER_POINTS + LOSE_POINTS; },
      // scissors vs rock
      "C X" => { round_score = ROCK_POINTS + WIN_POINTS; },
      // scissors vs scissors
      "C Z" => { round_score = SCISSOR_POINTS + DRAW_POINTS; },
      &_ => println!("Missing case {}", line)
    }

    total_score += round_score;
  }

  total_score
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> i32 {
  const LOSE_POINTS: i32 = 0;
  const DRAW_POINTS: i32 = 3;
  const WIN_POINTS: i32 = 6;

  const ROCK_POINTS: i32 = 1;
  const PAPER_POINTS: i32 = 2;
  const SCISSOR_POINTS: i32 = 3;

  let mut total_score: i32 = 0;
  let mut round_score: i32 = 0;

  for line in input.lines() {
    match line {
      // rock vs rock
      "A Y" => { round_score = ROCK_POINTS + DRAW_POINTS; },
      // rock vs scissors
      "A X" => { round_score = SCISSOR_POINTS + LOSE_POINTS; },
      // rock vs paper
      "A Z" => { round_score = PAPER_POINTS + WIN_POINTS; },
      // paper vs paper
      "B Y" => { round_score = PAPER_POINTS + DRAW_POINTS; },
      // paper vs rock
      "B X" => { round_score = ROCK_POINTS + LOSE_POINTS; },
      // paper vs scissors
      "B Z" => { round_score = SCISSOR_POINTS + WIN_POINTS; },
      // scissors vs scissors
      "C Y" => { round_score = SCISSOR_POINTS + DRAW_POINTS; },
      // scissors vs paper
      "C X" => { round_score = PAPER_POINTS + LOSE_POINTS; },
      // scissors vs rock
      "C Z" => { round_score = ROCK_POINTS + WIN_POINTS; },
      &_ => println!("Missing case {}", line)
    }

    total_score += round_score;
  }

  total_score
}