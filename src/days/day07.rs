use std::fs::read_to_string;
use rustc_hash::FxHashMap;
use crate::{Solution, SolutionPair};
use crate::etc::{Grid, Point};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day07.txt").unwrap();
    let grid = Grid::from_str(&input);

    let (sol1, sol2) = simulate(&grid);

    (Solution::from(sol1), Solution::from(sol2))
}

fn simulate(grid: &Grid<char>) -> (u32, u64) {
    let mut splits = 0;
    let start = grid.find(&'S').unwrap();
    let mut rays = FxHashMap::default();
    rays.insert(start, 1);

    loop {
        let mut new_rays = FxHashMap::default();
        for (&ray, &count) in rays.iter() {
            let down = ray.down();
            match grid.get(down) {
                None => return (splits, rays.values().sum()), // went OOB, simulation ends
                Some('.') => *new_rays.entry(down).or_default() += count,
                Some('^') => {
                    *new_rays.entry(down.left()).or_default() += count;
                    *new_rays.entry(down.right()).or_default() += count;
                    splits += 1;
                },
                _ => unreachable!(),
            }
        }
        rays = new_rays;
    }
}
