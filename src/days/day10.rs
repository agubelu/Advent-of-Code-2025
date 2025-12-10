use std::fs::read_to_string;

use itertools::Itertools;
use lpsolve::prelude::*;

use crate::etc::Grid;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

struct Machine {
    target: u32,
    buttons_bits: Vec<u32>,
    buttons_ixs: Vec<Vec<usize>>,
    voltages: Vec<usize>,
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day10.txt").unwrap();
    let machines = input.lines().map(parse_machine).collect_vec();

    let sol1: usize = machines.iter().map(solve_machine_p1).sum();
    let sol2: usize = machines.iter().map(solve_machine_p2).sum();

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn solve_machine_p1(m: &Machine) -> usize {
    for k in 1..=m.buttons_bits.len() {
        for comb in m.buttons_bits.iter().copied().combinations(k) {
            let val = comb.iter().copied().reduce(|a, b| a ^ b).unwrap();
            if val == m.target {
                return k;
            }
        }
    }
    unreachable!()
}

fn solve_machine_p2(m: &Machine) -> usize {
    let n_cols = m.buttons_ixs.len();
    let mut problem = Problem::builder()
        .verbosity(Verbosity::Critical) // Make it shut up
        .cols(n_cols as i32)
        .min(&vec![1.0; n_cols])
        .non_negative_integers();


    let mut mat = Grid::new(n_cols, m.voltages.len(), 0.0);
    for (x, ls) in m.buttons_ixs.iter().enumerate() {
        for &y in ls {
            mat[(x, y)] = 1.0;
        }
    }

    for (row, target) in mat.rows().zip(&m.voltages) {
        problem = problem.eq(row, *target as f64);
    }

    problem.solve().unwrap().objective_value().round() as usize
}

///////////////////////////////////////////////////////////////////////////////

fn parse_machine(line: &str) -> Machine {
    let mut parts = line.split(' ').collect_vec();
    let voltages = parse_values(parts.pop().unwrap());
    let target = parse_target(parts[0]);
    let buttons_bits = parts[1..].iter().copied().map(parse_button_bits).collect_vec();
    let buttons_ixs = parts[1..].iter().copied().map(parse_values).collect_vec();
    Machine { target, buttons_bits, buttons_ixs, voltages }
}

fn parse_target(target: &str) -> u32 {
    let inner = &target[1..target.len()-1];
    inner.char_indices().map(|(i, ch)| {
        (ch == '#') as u32 * (1 << i)
    }).reduce(|a, b| a | b).unwrap()
}

fn parse_button_bits(button: &str) -> u32 {
    let inner = &button[1..button.len()-1];
    inner.split(',').map(|ch| {
        let ix: u32 = ch.parse().unwrap();
        1 << ix
    }).reduce(|a, b| a | b).unwrap()
}

fn parse_values(button: &str) -> Vec<usize> {
    let inner = &button[1..button.len()-1];
    inner.split(',').map(|ch| ch.parse().unwrap()).collect()
}
