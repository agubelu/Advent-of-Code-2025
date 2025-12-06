use itertools::Itertools;
use crate::etc::Grid;
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Problem {
    numbers: Vec<u64>,
    op: char,
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day06.txt").unwrap();
    let (lines, ops) = read_input(&input);

    let problems_p1 = parse_data_p1(&lines, &ops);
    let problems_p2 = parse_data_p2(&lines, &ops);

    let sol1: u64 = problems_p1.iter().map(|p| p.solve()).sum();
    let sol2: u64 = problems_p2.iter().map(|p| p.solve()).sum();

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn read_input(input: &str) -> (Vec<&str>, Vec<char>) {
    let mut lines = input.lines().collect_vec();
    let ops = lines.pop().unwrap().chars().filter(|ch| !ch.is_whitespace()).collect_vec();
    (lines, ops)
}

fn parse_data_p1(lines: &[&str], ops: &[char]) -> Vec<Problem> {
    let mut number_iters = lines.iter().map(|line| line.split_whitespace()).collect_vec();
    let mut problems = vec![];

    for &op in ops {
        let numbers = number_iters.iter_mut().map(|it| it.next().unwrap().parse().unwrap()).collect_vec();
        problems.push(Problem{ numbers, op });
    }

    problems
}

fn parse_data_p2(lines: &[&str], ops: &[char]) -> Vec<Problem> {
    let width = lines[0].len();
    let data = lines.iter().flat_map(|line| line.chars()).filter(|&ch| ch != '\n').collect_vec();
    let mut ops_rev = ops.iter().copied().rev();

    let mut grid = Grid::from_data(width, data.len() / width, data);
    grid.rotate_left();

    let mut problems = vec![];
    let mut numbers = vec![];

    for line in grid.rows() {
        if let Some(n) = parse_number(line) {
            numbers.push(n);
        } else {
            problems.push(Problem { numbers, op: ops_rev.next().unwrap() });
            numbers = vec![];
        }
    }

    // Flush the final problem
    if !numbers.is_empty() {
        problems.push(Problem { numbers, op: ops_rev.next().unwrap() });
    }

    problems
}

fn parse_number(line: &[char]) -> Option<u64> {
    let mut res = 0;
    let mut any = false; // This allows 0's in the input numbers

    for ch in line {
        if let Some(d) = ch.to_digit(10) {
            any = true;
            res = res * 10 + d as u64;
        }
    }

    if any { Some(res) } else { None }
}

impl Problem {
    fn solve(&self) -> u64 {
        let op = match self.op {
            '+' => |a, b| a + b,
            '*' => |a, b| a * b,
             _  => unimplemented!(),
        };
        self.numbers.iter().copied().reduce(op).unwrap()
    }
}
