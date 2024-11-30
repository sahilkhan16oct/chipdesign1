use foundation::i_overlay::i_float::point::IntPoint;
use foundation::i_overlay::i_shape::int::shape::IntShape;
use foundation::task::orientation::Orientation;
use crate::solution::edge::{CreatEdges, Edge};
use crate::solution::solver::{Condition};
use crate::solution::vedge::{Unionable, VEdge};

pub trait MinMaxWidth {
    fn find_xy_problems(&self, orientation: Orientation, condition: Condition) -> Vec<Edge>;
}

impl MinMaxWidth for IntShape {
    fn find_xy_problems(&self, orientation: Orientation, condition: Condition) -> Vec<Edge> {
        let invert = if orientation == Orientation::Horizontal { true } else { false };
        let s_res = slice(&self, invert);
        let edges = s_res.0;
        let x_list = s_res.1;

        let mut problems = test_x(edges, x_list, condition);

        if invert {
            for problem in problems.iter_mut() {
                problem.invert()
            }
        }

        problems
    }
}

fn slice(shape: &IntShape, invert: bool) -> (Vec<Edge>, Vec<i32>) {
    let mut edges: Vec<Edge>;
    if shape.len() == 1 {
        edges = shape[0].create_edges(invert);
    } else {
        edges = Vec::with_capacity(2 * shape.len());
        for path in shape.iter() {
            let sub_edges = path.create_edges(invert);
            edges.extend(sub_edges);
        }
    }

    let mut i = 0;
    let mut x_list = Vec::with_capacity(2 * edges.len());
    while i < edges.len() {
        let e = edges[i];
        if e.is_vertical() {
            edges.swap_remove(i);
        } else {
            x_list.push(e.a.x);
            x_list.push(e.b.x);
            i += 1
        }
    }

    edges.sort_by(|e0, e1| e0.a.x.cmp(&e1.a.x));
    x_list.sort();

    let mut filter = Vec::with_capacity(x_list.len());
    let mut x0 = x_list[0];
    filter.push(x0);
    for x in x_list.into_iter() {
        if x != x0 {
            x0 = x;
            filter.push(x);
        }
    }

    return (edges, filter);
}

fn test_x(edges: Vec<Edge>, x_list: Vec<i32>, condition: Condition) -> Vec<Edge> {
    let capacity = ((2.5 * (edges.len() as f64).sqrt()) as usize).min(edges.len());
    let mut scan_list = Vec::with_capacity(capacity);
    let mut ed_res = Vec::new();

    let mut j = 0;

    for x in x_list.into_iter() {
        let e_list0 = test(&scan_list, x, condition);

        // remove old
        let mut i = 0;
        while i < scan_list.len() {
            let bx = scan_list[i].b.x;
            if bx <= x {
                scan_list.swap_remove(i);
            } else {
                i += 1;
            }
        }

        // add new
        while j < edges.len() {
            let e = edges[j];
            let ax = e.a.x;
            if ax <= x {
                scan_list.push(e);
                j += 1;
            } else {
                break;
            }
        }

        let mut e_list1 = test(&scan_list, x, condition);

        if !e_list0.is_empty() && !e_list1.is_empty() {
            e_list1.extend(e_list0);
            e_list1.union();
            add(&mut ed_res, x, &e_list1);
        } else if !e_list0.is_empty() {
            add(&mut ed_res, x, &e_list0);
        } else if !e_list1.is_empty() {
            add(&mut ed_res, x, &e_list1);
        }
    }

    ed_res
}

fn test(edges: &Vec<Edge>, x: i32, condition: Condition) -> Vec<VEdge> {
    if edges.len() < 2 {
        return Vec::new();
    }

    let mut y_list = Vec::with_capacity(edges.len());
    for e in edges.iter() {
        let y = e.vertical_intersection(x);
        y_list.push(y);
    }

    y_list.sort();

    let mut vt_list = Vec::with_capacity(edges.len());
    let mut pe = VEdge::with_y(y_list[0]);

    let mut i = 0;
    while i + 1 < y_list.len() {
        let y0 = y_list[i];
        let y1 = y_list[i + 1];
        let dy = y1 - y0;

        if condition.test(dy) {
            let ei = VEdge::new(y0, y1);

            // try to merge
            if !pe.join(ei) {
                if !pe.is_zero() {
                    vt_list.push(pe)
                }
                pe = ei;
            }
        }

        i += 2;
    }

    if !pe.is_zero() {
        vt_list.push(pe)
    }

    vt_list
}

fn add(edges: &mut Vec<Edge>, x: i32, vt_list: &Vec<VEdge>) {
    for v in vt_list.iter() {
        let e = Edge::new(IntPoint::new(x, v.y0), IntPoint::new(x, v.y1));
        edges.push(e);
    }
}

impl Condition {
    fn test(&self, y: i32) -> bool {
        match self {
            Condition::Max(value) => {
                y > *value
            }
            Condition::Min(value) => {
                y < *value
            }
        }
    }
}