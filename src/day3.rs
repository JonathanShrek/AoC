use core::{iter::Iterator, str::Lines};
use regex::{CaptureMatches, Match, Regex};

lazy_static! {
    static ref NUM_RE: Regex = Regex::new(r"\d+").unwrap();
    static ref SYM_RE: Regex = Regex::new(r"[^.\d]").unwrap();
}

// Iterator for (above, cur, below) line windows

type LineWindow<'a> = (Option<&'a str>, &'a str, Option<&'a str>);

struct Windows<'a> {
    lines: Lines<'a>,
    buf: [Option<&'a str>; 2],
}

impl<'a> Iterator for Windows<'a> {
    type Item = LineWindow<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let window = (self.buf[0].take(), self.buf[1].take()?, self.lines.next());
        self.buf = [Some(window.1), window.2];
        Some(window)
    }
}

fn windows(input: &str) -> Windows {
    let mut lines = input.lines();
    let buf = [None, lines.next()];
    Windows { lines, buf }
}

// Iterator for matches of a pattern around a given string

struct Adjacent<'a, 'b> {
    cur: CaptureMatches<'a, 'b>,
    above: Option<CaptureMatches<'a, 'b>>,
    below: Option<CaptureMatches<'a, 'b>>,
    range: (usize, usize),
}

fn find_adjacent<'a, 'b>(
    pat: &'a Regex,
    w: LineWindow<'b>,
    range: (usize, usize),
) -> Adjacent<'a, 'b> {
    let cur = pat.captures_iter(w.1);
    let above = w.0.map(|above| pat.captures_iter(above));
    let below = w.2.map(|below| pat.captures_iter(below));
    Adjacent { cur, above, below, range }
}

impl<'a, 'b> Iterator for Adjacent<'a, 'b> {
    type Item = &'b str;

    fn next(&mut self) -> Option<Self::Item> {
        for cap in &mut self.cur {
            let cap = cap.get(0).unwrap();
            if cap.start() == self.range.1 || cap.end() == self.range.0 {
                return Some(cap.as_str());
            }
        }
        for caps in [&mut self.above, &mut self.below].into_iter().flatten() {
            for cap in caps {
                let cap = cap.get(0).unwrap();
                if cap.start() <= self.range.1 && cap.end() >= self.range.0 {
                    return Some(cap.as_str());
                }
            }
        }
        None
    }
}

// #[aoc_generator(day3)]
// pub fn input_generator(input: &str) -> HashMap<u32, Vec<char>> {
// }

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> i64 { 
  let has_adj_sym = |w: LineWindow, around: &Match| {
    find_adjacent(&SYM_RE, w, (around.start(), around.end())).next().is_some()
  };

  let window_sum = |w: LineWindow| -> i64 {
    let nums = NUM_RE.captures_iter(w.1).map(|cap| cap.get(0).unwrap());
    let nums_touching_sym = nums.filter(|cap| has_adj_sym(w, cap));
    nums_touching_sym.map(|cap| cap.as_str().parse::<i64>().unwrap()).sum()
  };

  windows(input).map(window_sum).sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> i64 {
  let gear_ratio = |adj_nums: Adjacent| -> Option<i64> {
        let mut nums = adj_nums.map(|cap| cap.parse::<i64>().unwrap());
        let ratio = [nums.next()?, nums.next()?].into_iter().product();
        nums.next().xor(Some(ratio))
    };

    let window_sum = |w: LineWindow| -> i64 {
        let star_pos = w.1.chars().enumerate().filter(|p| p.1 == '*').map(|p| p.0);
        let ratio_at = |i: usize| gear_ratio(find_adjacent(&NUM_RE, w, (i, i + 1)));
        star_pos.flat_map(ratio_at).sum()
    };

    windows(input).map(window_sum).sum()
}