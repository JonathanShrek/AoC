#[derive(Debug, Clone)]
pub struct Game {
  id: u32,
  rounds: Vec<Round>,
}

#[derive(Debug, Clone)]
pub struct Round {
  red: u32,
  blue: u32,
  green: u32,
}

impl Game {
  pub fn new(id: u32) -> Self {
    Self {
      id,
      rounds: Vec::new(),
    }
  }

  pub fn add_round(&mut self, red: u32, blue: u32, green: u32) {
    let round = Round { red, blue, green };
    self.rounds.push(round);
  }

  pub fn print_rounds(&self) {
    for round in &self.rounds {
      println!("id {}: {:?}", &self.id, round);
    }
  }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
  let mut games: Vec<Game> = Vec::new();

  input
    .lines()
    .for_each(|line| {
      let mut line = line.split(":");
      let game_str = line.next().unwrap_or_default();
      let game_id_str = game_str.split(" ").last().unwrap_or_default();
      let game_id = game_id_str.parse::<u32>().unwrap_or_default();

      let round_str = line.next().unwrap_or_default();
      let round_str_vec = round_str.split(":").collect::<Vec<&str>>();

      let mut game = Game::new(game_id);

      round_str_vec.iter().for_each(|round_str| {
        let round_str_vec = round_str.trim().split("; ").collect::<Vec<&str>>();

        round_str_vec.iter().for_each(|round| {
          let cubes_str_vec = round.split(", ").collect::<Vec<&str>>();

          let mut red: u32 = 0;
          let mut blue: u32 = 0;
          let mut green: u32 = 0;
          cubes_str_vec.iter().for_each(|cube_str| {
            let cube_vec = cube_str.trim().split(" ").collect::<Vec<&str>>();
            let cube_color = cube_vec[1];
            let cube_count_str = cube_vec[0];

            match cube_color {
               "red" => {
                  red = cube_count_str.parse::<u32>().unwrap_or_default();
               } 
               "blue" => {
                  blue = cube_count_str.parse::<u32>().unwrap_or_default();
               }
               "green" => {
                  green = cube_count_str.parse::<u32>().unwrap_or_default();
               }
               _ => {}
            }
          });

          game.add_round(red, blue, green);
        });

        games.push(game.clone());
      });
    });

    games
}

#[aoc(day2, part1)]
pub fn solve_part1(data: &Vec<Game>) -> u32 { 
  let mut answer: u32 = 0;

  for game in data {
    let mut valid_round = true;
    for round in &game.rounds {
      if round.red > 12 {
        valid_round = false;
      }

      if round.green > 13 {
        valid_round = false;
      }

      if round.blue > 14 {
        valid_round = false;
      }
    }

    if valid_round {
      answer += game.id;
    }
  }

  answer
}

#[aoc(day2, part2)]
pub fn solve_part2(data: &Vec<Game>) -> u32 {
  let mut answer: u32 = 0;

  for game in data {
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;
    for round in &game.rounds {
      if round.red > red {
        red = round.red;
      }

      if round.blue > blue {
        blue = round.blue;
      }

      if round.green > green {
        green = round.green;
      }
    }

    let power = red * blue * green;

    answer += power;
  }

  answer
}