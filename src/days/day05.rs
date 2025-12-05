use crate::{Solution, SolutionPair};
use crate::etc::DOUBLE_NEWLINE;
use itertools::Itertools;
use std::fs::read_to_string;
use Marker::*;

///////////////////////////////////////////////////////////////////////////////

type Range = (u64, u64);

#[derive(PartialEq, Eq)]
enum Marker { Start, End }

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day05.txt").unwrap();
    let (ranges, ids) = parse_input(&input);

    let sol1 = ids.into_iter().filter(|&x| is_fresh(x, &ranges)).count();
    let sol2 = count_common_values(&ranges);

    (Solution::from(sol1), Solution::from(sol2))
}

fn is_fresh(id: u64, ranges: &[Range]) -> bool {
    ranges.iter().any(|&(min, max)| min <= id && id <= max)
}

fn count_common_values(ranges: &[Range]) -> u64 {
    let (mut start, mut active, mut total) = (0, 0, 0);
    let mut flat = ranges.iter().flat_map(|&(min, max)| [(min, Start), (max + 1, End)]).collect_vec();
    flat.sort_by_key(|x| x.0);

    for (val, marker) in flat {
        if marker == Start {
            if active == 0 {
                start = val;
            }
            active += 1;
        } else { // marker == End
            active -= 1;
            if active == 0 {
                total += val - start;
            }
        }
    }

    total
}

fn parse_input(input: &str) -> (Vec<Range>, Vec<u64>) {
    let (head, tail) = input.split_once(DOUBLE_NEWLINE).unwrap();
    let ranges = head.lines().map(parse_range).collect();
    let ids = tail.lines().map(|x| x.parse().unwrap()).collect();
    (ranges, ids)
}

fn parse_range(line: &str) -> Range {
    line.split('-').map(|x| x.parse().unwrap()).collect_tuple().unwrap()
}
