use itertools::Itertools;

use crate::{Solution, SolutionPair};
use crate::etc::{Grid, Point};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day04.txt").unwrap();
    let mut grid = Grid::from_str(&input);

    let sol1 = grid.enumerate().filter(|&(p, _)| can_be_removed(&grid, p)).count();
    let sol2 = cleanup(&mut grid);

    (Solution::from(sol1), Solution::from(sol2))
}

fn can_be_removed(grid: &Grid<char>, point: Point) -> bool {
    grid[point] == '@' &&
    point.neighbors_diag()
         .into_iter()
         .map(|p| grid.get_or(p, '.'))
         .filter(|c| *c == '@')
         .count() < 4
}

fn cleanup(grid: &mut Grid<char>) -> u32 {
    let mut removed = 0;
    let mut to_visit = grid.enumerate().map(|x| x.0).collect_vec();

    while let Some(point) = to_visit.pop() {
        if can_be_removed(grid, point) {
            removed += 1;
            grid[point] = '.';
            to_visit.extend(
                point.neighbors_diag()
                     .into_iter()
                     .filter(|&p| grid.get_or(p, '.') == '@')
            );
        }
    }
    removed
}
