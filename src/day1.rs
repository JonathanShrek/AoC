// #[aoc_generator(day1)]
// pub fn input_generator(word: &str) -> Option<&'static str> {
// }

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
  let sum: u32 = input
    .lines()
    .map(|line| {
      let first = line
        .chars()
        .find(|d| d.is_ascii_digit())
        .expect("first digit found");
      let last = line
        .chars()
        .filter(|d| d.is_ascii_digit())
        .last()
        .expect("last digit found");

      format!("{first}{last}")
        .parse::<u32>()
        .expect("digits can be converted to u32")
    })
    .sum();

  sum
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
  let strings: Vec<&str> = vec![
      "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3",
      "4", "5", "6", "7", "8", "9",
  ];

  let sum: u32 = input
    .lines()
    .map(|line| {
      let min_idx = strings
        .clone()
        .into_iter()
        .filter_map(|substring| line.find(substring))
        .min()
        .expect("first index was found");
      let max_idx = strings
        .clone()
        .into_iter()
        .filter_map(|substring| line.rfind(substring))
        .max()
        .expect("last index was found");

      let first_digit = strings
        .iter()
        .find(|&&substring| line[min_idx..].starts_with(substring))
        .and_then(|&str| string_to_digit(str))
        .expect("first digit was found");

      let last_digit = strings
        .iter()
        .find(|&&substring| line[max_idx..].starts_with(substring))
        .and_then(|&str| string_to_digit(str))
        .expect("last digit was found");

      format!("{first_digit}{last_digit}")
        .parse::<u32>()
        .expect("cannot convert digits to u32")
    })
    .sum();

  sum
}

fn string_to_digit(string: &str) -> Option<u32> {
  match string {
    "one" | "1" => Some(1),
    "two" | "2" => Some(2),
    "three" | "3" => Some(3),
    "four" | "4" => Some(4),
    "five" | "5" => Some(5),
    "six" | "6" => Some(6),
    "seven" | "7" => Some(7),
    "eight" | "8" => Some(8),
    "nine" | "9" => Some(9),
    _ => None,
  }
}