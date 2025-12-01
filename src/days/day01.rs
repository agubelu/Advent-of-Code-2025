use itertools::Itertools;

use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day01.txt").unwrap();
    let moves = input.lines().map(parse_input_line).collect_vec();

    let mut dial = 50;
    let (mut sol1, mut sol2) = (0, 0);

    for mv in moves {
        if mv >= 0 {
            sol2 += ((dial + mv) as f32 / 100.0).floor() as i32;
        } else {
            let offset = if dial == 0 { 100 } else { dial };
            sol2 += ((offset + mv) as f32 / -100.0).floor() as i32 + 1;
        }
        dial = (dial + mv).rem_euclid(100);
        if dial == 0 {
            sol1 += 1;
        }
    }

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse_input_line(line: &str) -> i32 {
    let amt: i32 = line[1..].parse().unwrap();
    if line.starts_with('R') { amt } else { -amt }
}