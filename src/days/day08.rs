use std::fs::read_to_string;
use std::cmp::min;

use counter::Counter;
use itertools::Itertools;
use petgraph::prelude::*;
use petgraph::algo::connected_components;
use petgraph::unionfind::UnionFind;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

type PairDist = (usize, usize);
struct Junction {
    x: i64,
    y: i64,
    z: i64,
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day08.txt").unwrap();

    let boxes = input.lines().map(parse_junction).collect_vec();
    let distances = compute_distances(&boxes);

    let sol1 = solve_p1(&distances);
    let sol2 = solve_p2(&distances, &boxes);

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn solve_p1(dists: &[PairDist]) -> u64 {
    // Adaptation of petgraph's `algo::connected_components` to get the sizes of such components
    // instead of just how many of them there are
    let graph = make_graph(dists, 1000);

    let mut node_sets = UnionFind::new(graph.node_count());
    for edge in graph.edge_references() {
        let (a, b) = (edge.source(), edge.target());
        node_sets.union(a.index(), b.index());
    }

    let ccs = node_sets.into_labeling().into_iter().collect::<Counter<_, u64>>();
    ccs.values().sorted().rev().take(3).product()
}

fn solve_p2(dists: &[PairDist], boxes: &[Junction]) -> i64 {
    let mut best = usize::MAX;
    let (mut lo, mut hi) = (0, dists.len());

    while lo <= hi {
        let mid = (lo + hi) / 2;
        let g = make_graph(dists, mid);

        if g.node_count() < boxes.len() || connected_components(&g) > 1 {
            lo = mid + 1; // Too low
        } else {
            hi = mid - 1; // Within range
            best = min(best, mid);
        }
    }

    let edge = dists[best-1];
    boxes[edge.0].x * boxes[edge.1].x
}

fn make_graph(dists: &[PairDist], n: usize) -> UnGraph<(), (), usize> {
    UnGraph::from_edges(dists.iter().take(n).map(|x| (x.0, x.1)))
}

fn compute_distances(boxes: &[Junction]) -> Vec<PairDist> {
    (0..boxes.len())
        .tuple_combinations()
        .sorted_by_cached_key(|&(a, b)| distance(&boxes[a], &boxes[b]))
        .collect_vec()
}

fn distance(p: &Junction, q: &Junction) -> i64 {
    (p.x - q.x).pow(2) + (p.y - q.y).pow(2) + (p.z - q.z).pow(2)
}

fn parse_junction(line: &str) -> Junction {
    let (x, y, z) = line.split(',').map(|x| x.parse().unwrap()).collect_tuple().unwrap();
    Junction { x, y, z }
}
