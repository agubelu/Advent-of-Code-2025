use crate::{Solution, SolutionPair};
use rayon::prelude::*;
use itertools::Itertools;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let ranges: Vec<(i64, i64)> = read_to_string("input/day02.txt").unwrap().trim()
        .split(',')
        .map(|s| s.split('-').map(|x| x.parse().unwrap()).collect_tuple().unwrap())
        .collect();

    let (sol1, sol2) = ranges.into_par_iter()
        .map(sum_invalid_values)
        .reduce_with(|a, b| (a.0 + b.0, a.1 + b.1))
        .unwrap();

    (Solution::from(sol1), Solution::from(sol2))
}

fn sum_invalid_values((min, max): (i64, i64)) -> (i64, i64) {
    let (mut p1, mut p2) = (0, 0);
    for val in min..=max {
        let digs = num_digits(val);
        if digs % 2 == 0 && is_repetition(val, digs / 2) {
            p1 += val;
        }
        if (1..=digs / 2).any(|n| is_repetition(val, n)) {
            p2 += val;
        }
    }
    (p1, p2)
}

fn is_repetition(val: i64, n: u32) -> bool {
    // Checks whether `val` is composed of at least 2 repetitions of its `n` first digits
    let digits = num_digits(val);

    if digits <= n || digits % n != 0 {
        return false;
    }

    let div = 10_i64.pow(n);
    let base = val % div;
    val == (0..digits / n).map(|i| base * 10_i64.pow(i*n)).sum()
}

fn num_digits(val: i64) -> u32 {
    (val as f32).log10().floor() as u32 + 1
}
