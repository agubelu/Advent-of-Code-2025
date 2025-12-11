use std::fs::read_to_string;

use itertools::Itertools;
use petgraph::prelude::*;
use petgraph::visit::{Topo, Walker};

use crate::etc::IDAssigner;
use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

struct Ctx<'a> {
    graph: DiGraph<(), ()>,
    ids: IDAssigner<&'a str, u32>,
    nodes: Vec<NodeIndex>,
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day11.txt").unwrap();
    let mut ctx = build_ctx(&input);

    let sol1 = count_paths(&mut ctx, "you", "out");
    let sol2 = solve_part2(&mut ctx);

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn count_paths<'a>(ctx: &mut Ctx<'a>, from: &'a str, to: &'a str) -> u64 {
    let mut counts = vec![0; ctx.nodes.len()];
    counts[ctx.ids.get_id(from) as usize] = 1;

    for l in &ctx.nodes {
        for r in ctx.graph.neighbors_directed(*l, Direction::Outgoing) {
            counts[r.index()] += counts[l.index()];
        }
    }

    counts[ctx.ids.get_id(to) as usize]
}

fn solve_part2(ctx: &mut Ctx) -> u64 {
    let svr_dac = count_paths(ctx, "svr", "dac");
    let svr_fft = count_paths(ctx, "svr", "fft");
    let dac_fft = count_paths(ctx, "dac", "fft");
    let fft_dac = count_paths(ctx, "fft", "dac");
    let dac_out = count_paths(ctx, "dac", "out");
    let fft_out = count_paths(ctx, "fft", "out");

    svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out
}

///////////////////////////////////////////////////////////////////////////////

fn build_ctx(input: &str) -> Ctx {
    let mut ids = IDAssigner::new();
    let mut edges = vec![];

    for line in input.lines() {
        let (in_node, outs) = line.split_once(": ").unwrap();
        let in_id = ids.get_id(in_node);
        edges.extend(outs.split(' ').map(
            |out| (in_id, ids.get_id(out), ())
        ));
    }

    let graph = DiGraph::from_edges(&edges);
    let nodes = Topo::new(&graph).iter(&graph).collect_vec();
    Ctx { graph, ids, nodes }
}
