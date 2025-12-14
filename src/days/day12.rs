use std::fs::read_to_string;
use std::str::Lines;
use itertools::Itertools;
use crate::etc::Grid;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type Present = Grid<char>;
struct Region {
    width: usize,
    height: usize,
    presents: Vec<usize>,
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day12.txt").unwrap();
    let (presents, regions) = parse_input(&input);
    let present_sizes = presents.iter().map(
        |p| p.find_all(&'#').count()
    ).collect_vec();

    let sol1 = regions.into_iter().filter(|r| can_fit(r, &present_sizes)).count();
    let sol2 = "🎄";

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn can_fit(region: &Region, present_sizes: &[usize]) -> bool {
    let total_area = region.width * region.height;
    let usable_area = (region.width - (region.width % 3)) * (region.height - (region.height % 3));

    let num_presents = region.presents.iter().sum::<usize>();
    let presents_area = (0..present_sizes.len())
        .map(|i| region.presents[i] * present_sizes[i]).sum();

    if usable_area >= 9 * num_presents {
        true   // trivial yes
    } else if total_area < presents_area {
        false  // trivial no
    } else {
        unimplemented!()  // we'd need to do actual calculations here
    }
}

///////////////////////////////////////////////////////////////////////////////

fn parse_input(input: &str) -> (Vec<Present>, Vec<Region>) {
    let mut lines = input.lines();
    let mut presents = vec![];

    while !lines.next().unwrap().contains('x') {
        presents.push(parse_present(&mut lines));
    }

    let regions = lines.map(parse_region).collect_vec();
    (presents, regions)
}

fn parse_present(lines: &mut Lines<'_>) -> Present {
    let mut s = String::new();
    let mut line;

    while {line = lines.next().unwrap(); !line.is_empty()} {
        s.push_str(&format!("{line}\n"));
    }

    Present::from_str(&s)
}

fn parse_region(line: &str) -> Region {
    let (l, r) = line.split_once(": ").unwrap();
    let (width, height) = l.split('x').map(|x| x.parse().unwrap()).collect_tuple().unwrap();
    let presents = r.split(' ').map(|x| x.parse().unwrap()).collect_vec();
    Region { width, height, presents }
}
