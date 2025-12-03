use crate::{Solution, SolutionPair};
use crate::etc::Grid;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day03.txt").unwrap();
    let grid = Grid::map_from_str(&input, |d| d.to_digit(10).unwrap() as u64);

    let sol1: u64 = grid.rows().map(|row| best_battery_combination(row, 2)).sum();
    let sol2: u64 = grid.rows().map(|row| best_battery_combination(row, 12)).sum();

    (Solution::from(sol1), Solution::from(sol2))
}

fn best_battery_combination(ls: &[u64], k: usize) -> u64 {
    let ix = choose_digit(ls, k);
    if k == 1 {
        return ls[ix];
    }

    let val = ls[ix] * 10_u64.pow(k as u32 - 1);
    let rest = &ls[ix+1..];
    val + best_battery_combination(rest, k - 1)
}

fn choose_digit(ls: &[u64], rem: usize) -> usize {
    // Leave out the last `rem - 1` items (so we can still pick exactly `rem` elements),
    // look for the highest digit in the remaining list,
    // and return the first appearance of the highest digit (left-to-right)
    // This works because picking the highest most significant digit always guarantees the highest possible value,
    // and picking the leftmost appearance of such digit leaves more options open for the next digit.
    ls.iter()
      .take(ls.len() - rem + 1)
      .enumerate()
      .reduce(|a, b| if b.1 > a.1 {b} else {a})
      .unwrap().0
}
