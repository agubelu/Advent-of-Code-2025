use std::cmp::{max, min};
use std::fs::read_to_string;

use geo::{Covers, LineString, Point as Pnt, Polygon};
use itertools::Itertools;
use rayon::prelude::*;

use crate::{Solution, SolutionPair};
use Segment::*;

///////////////////////////////////////////////////////////////////////////////

type Point = (i64, i64);
enum Segment {
    Vertical { x: i64, start: i64, end: i64 },
    Horizontal { y: i64, start: i64, end: i64 },
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day09.txt").unwrap();

    let points = input.lines().map(parse_point).collect_vec();
    let segments = build_segments(&points);
    let poly = build_polygon(&points);

    let sol1 = points.iter()
        .tuple_combinations()
        .map(rect_area)
        .max().unwrap();

    let sol2 = points.iter()
        .tuple_combinations()
        .par_bridge()
        .filter(|&(p1, p2)| rect_within_polygon(*p1, *p2, &segments, &poly))
        .map(rect_area)
        .max().unwrap();

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn build_segments(points: &[Point]) -> Vec<Segment> {
    (0..points.len())
        .map(|i| make_segment(points[i], points[(i + 1) % points.len()]))
        .collect()
}

fn build_polygon(points: &[Point]) -> Polygon<i64> {
    Polygon::new(
        LineString::from_iter(points.iter().copied()),
        vec![]
    )
}

fn make_segment(p1: Point, p2: Point) -> Segment {
    if p1.0 == p2.0 {
        Vertical { x: p1.0, start: min(p1.1, p2.1), end: max(p1.1, p2.1) }
    } else {
        Horizontal{ y: p1.1, start: min(p1.0, p2.0), end: max(p1.0, p2.0) }
    }
}

fn make_rect_segments(p1: Point, p2: Point) -> Vec<Segment> {
    if p1.0 == p2.0 || p1.1 == p2.1 {
        vec![make_segment(p1, p2)]
    } else {
        vec![
            make_segment((p1.0, p1.1), (p2.0, p1.1)),
            make_segment((p2.0, p1.1), (p2.0, p2.1)),
            make_segment((p2.0, p2.1), (p1.0, p2.1)),
            make_segment((p1.0, p2.1), (p1.0, p1.1)),
        ]
    }
}

fn rect_area((p1, p2): (&Point, &Point)) -> i64 {
    ((p1.0 - p2.0).abs() + 1) * ((p1.1 - p2.1).abs() + 1)
}

fn rect_within_polygon(p1: Point, p2: Point, segs: &[Segment], poly: &Polygon<i64>) -> bool {
    let rect_segs = make_rect_segments(p1, p2);
    !rect_segs.iter().any(|s_rect| {
        segs.iter().any(|s_poly| segments_intersect(s_rect, s_poly, poly))
    })
}

fn segments_intersect(s_rect: &Segment, s_poly: &Segment, poly: &Polygon<i64>) -> bool {
    // Checks whether a line segment of the rectangle and the polygon intersect
    // If the two segments are in the same direction, an intersection only happens
    // if the rectangle's segment has any part outside the polygon
    match (s_rect, s_poly) {
        (Vertical{x: xr, start: sr, end: er}, Vertical{x: xp, start: sp, end: ep}) => {
            xr == xp && !(er < sp || sr > ep) && {
                let p1 = Pnt::new(*xr, *sr);
                let p2 = Pnt::new(*xr, *er);
                !poly.covers(&p1) || !poly.covers(&p2)
            }
        },
        (Horizontal{y: yr, start: sr, end: er}, Horizontal{y: yp, start: sp, end: ep}) => {
            yr == yp && !(er < sp || sr > ep) && {
                let p1 = Pnt::new(*sr, *yr);
                let p2 = Pnt::new(*er, *yr);
                !poly.covers(&p1) || !poly.covers(&p2)
            }
        },
        (Vertical{x: xr, start: sr, end: er}, Horizontal{y: yp, start: sp, end: ep}) => {
            yp > sr && yp < er && xr > sp && xr < ep
        },
        (Horizontal{y: yr, start: sr, end: er}, Vertical{x: xp, start: sp, end: ep}) => {
            xp > sr && xp < er && yr > sp && yr < ep
        },
    }

}

fn parse_point(line: &str) -> Point {
    line.split(',').map(|x| x.parse().unwrap()).collect_tuple().unwrap()
}
