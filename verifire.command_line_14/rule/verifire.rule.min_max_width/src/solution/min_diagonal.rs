use std::collections::HashSet;
use foundation::i_overlay::i_float::fix_vec::FixVec;
use foundation::i_overlay::i_float::point::IntPoint;
use foundation::i_overlay::i_shape::fix::shape::FixShape;
use foundation::i_overlay::line_range::LineRange;
use foundation::space::line_container::LineContainer;
use foundation::space::line_segment::LineSegment;
use foundation::space::line_space::{IntExtensions, LineSpace};
use foundation::space::line_range::LineRangeExt;

use crate::solution::edge::Edge;

pub trait MinDiagonal {
    fn find_diagonal_problems(&self, min_length: i32) -> Vec<Edge>;
}

#[derive(Debug, Clone)]
struct Dist {
    id: Option<usize>,
    ignore: Option<usize>,
    x_min: i64,
    x_max: i64,
    a: FixVec,
    b: FixVec,
}

impl Dist {
    fn new(id: Option<usize>, ignore: Option<usize>, a: FixVec, b: FixVec) -> Dist {
        let (min_x, max_x) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };
        Dist { id, ignore, x_min: min_x, x_max: max_x, a, b }
    }

    fn range(&self) -> LineRange {
        if self.a.y < self.b.y {
            LineRange { min: self.a.y as i32, max: self.b.y as i32 }
        } else {
            LineRange { min: self.b.y as i32, max: self.a.y as i32 }
        }
    }

    fn is_cross(&self, dist: &Dist) -> bool {
        match (self.id, dist.id) {
            (None, None) | (Some(_), Some(_)) => false,
            _ => {
                if dist.ignore == self.id || self.ignore == dist.id {
                    return false;
                }

                if dist.a == self.a || dist.a == self.b || dist.b == self.a || dist.b == self.b {
                    return false;
                }

                if Dist::is_not_divide(self.a, self.b, dist.a, dist.b) == false ||
                    Dist::is_not_divide(dist.a, dist.b, self.a, self.b) == false {
                    return false;
                }

                true
            }
        }
    }

    fn is_not_divide(v0: FixVec, v1: FixVec, e0: FixVec, e1: FixVec) -> bool {
        let v = v0 - v1; // Assuming FixVec has a subtraction implementation
        let a = v.cross_product(v0 - e0); // Assuming FixVec has a method `unsafe_cross_product`
        let b = v.cross_product(v0 - e1);

        (a < 0 && b < 0) || (a > 0 && b > 0)
    }
}

struct ComplexIndex {
    path_index: usize,
    vert_index: usize,
    last_index: usize,
}

impl ComplexIndex {
    fn is_first(&self) -> bool {
        self.vert_index == 0
    }

    fn is_last(&self) -> bool {
        self.vert_index == self.last_index
    }

    fn is_neighbor(&self, other: &ComplexIndex) -> bool {
        self.path_index == other.path_index &&
            (self.vert_index + 1 == other.vert_index || self.is_last() && other.is_first() || other.vert_index + 1 == self.vert_index || other.is_last() && self.is_first())
    }
}

struct Vector {
    x_max: i64,
    x_min: i64,
    a: FixVec,
    b: FixVec,
    vector: FixVec,
    id: usize,
    index: ComplexIndex,
    length: f64,
    i_scale: f64,
}

impl Vector {
    fn new(a: FixVec, b: FixVec, index: ComplexIndex, id: usize) -> Self {
        let (x_max, x_min) = if a.x < b.x {
            (b.x, a.x)
        } else {
            (a.x, b.x)
        };

        let vector = b - a;
        let m = vector.sqr_length(); // Adjust method name as per Rust conventions
        let length = (m as f64).sqrt();
        let i_scale = 1.0 / length;

        Vector {
            x_max,
            x_min,
            a,
            b,
            vector,
            id,
            index,
            length,
            i_scale,
        }
    }

    fn range(&self) -> LineRange {
        if self.a.y < self.b.y {
            LineRange { min: self.a.y as i32, max: self.b.y as i32 }
        } else {
            LineRange { min: self.b.y as i32, max: self.a.y as i32 }
        }
    }
}

struct DScanList {
    space: LineSpace<usize>,
    delta: i32,
}

impl DScanList {
    fn new(delta: i32, vectors: &Vec<Vector>) -> Self {
        let mut y_min = i64::MAX;
        let mut y_max = i64::MIN;
        for vector in vectors.iter() {
            if vector.a.y > vector.b.y {
                y_min = y_min.min(vector.b.y);
                y_max = y_max.max(vector.a.y);
            } else {
                y_min = y_min.min(vector.a.y);
                y_max = y_max.max(vector.b.y);
            }
        }

        let max_level = ((vectors.len() as f64).sqrt() as usize).log_two();
        let range = LineRange { min: y_min as i32, max: y_max as i32 };

        Self { space: LineSpace::new(max_level, range), delta }
    }

    fn find_all_in_range(&mut self, range: LineRange, containers: &mut Vec<LineContainer<usize>>) {
        let delta_range = LineRange { min: range.min - self.delta, max: range.max + self.delta }.clamp(&self.space.indexer.range);
        self.space.all_in_range(delta_range, containers)
    }
}

struct FScanList {
    space: LineSpace<usize>,
}

impl FScanList {
    fn new(dists: &Vec<Dist>) -> Self {
        let mut y_min = i64::MAX;
        let mut y_max = i64::MIN;
        for dist in dists.iter() {
            if dist.a.y > dist.b.y {
                y_min = y_min.min(dist.b.y);
                y_max = y_max.max(dist.a.y);
            } else {
                y_min = y_min.min(dist.a.y);
                y_max = y_max.max(dist.b.y);
            }
        }

        let max_level = ((dists.len() as f64).sqrt() as usize).log_two();
        let range = LineRange { min: y_min as i32, max: y_max as i32 };

        Self { space: LineSpace::new(max_level, range) }
    }

    fn find_all_in_range(&mut self, range: LineRange, containers: &mut Vec<LineContainer<usize>>) {
        self.space.all_in_range(range, containers)
    }
}

impl MinDiagonal for FixShape {
    fn find_diagonal_problems(&self, min_length: i32) -> Vec<Edge> {
        let mut dist_list = find_raw_result(self, min_length as i64);
        filter(self, &mut dist_list)
    }
}

fn find_raw_result(shape: &FixShape, min_length: i64) -> Vec<Dist> {
    let vectors = build_vectors(shape);
    let mut scan_list = DScanList::new(min_length as i32, &vectors);
    let mut candidates = Vec::new();
    let mut remove_list = Vec::new();
    let mut dist_list = Vec::new();

    for i in 0..vectors.len() {
        let vi = &vectors[i];
        let x_min = vi.x_min - min_length;

        let range = vi.range();

        scan_list.find_all_in_range(range, &mut candidates);

        for candidate in candidates.iter() {
            let vj = &vectors[candidate.id];

            if vj.x_max < x_min {
                remove_list.push(candidate.index);
            } else {
                distance(shape, vi, vj, min_length, &mut dist_list);
            }
        }

        if !remove_list.is_empty() {
            scan_list.space.remove_indices(&mut remove_list);
            remove_list.clear();
        }

        candidates.clear();

        scan_list.space.insert(LineSegment { id: i, range });
    }

    dist_list
}

fn filter(shape: &FixShape, dist_list: &mut Vec<Dist>) -> Vec<Edge> {
    if dist_list.is_empty() {
        return Vec::new();
    }

    let mut not_valid_set = HashSet::new();

    let mut obstacles = build_obstacles(shape);
    dist_list.append(&mut obstacles);
    dist_list.sort_by(|a, b| a.x_min.partial_cmp(&b.x_min).unwrap());

    let mut scan_list = FScanList::new(dist_list);
    let mut candidates = Vec::new();
    let mut remove_list = Vec::new();

    for i in 0..dist_list.len() {
        let di = dist_list[i].clone();
        let x_min = di.x_min;
        let range = di.range();

        scan_list.find_all_in_range(range, &mut candidates);

        for candidate in candidates.iter_mut() {
            let j = candidate.id;
            let dj = &dist_list[j];

            if dj.x_max < x_min {
                remove_list.push(candidate.index);
            } else if di.is_cross(dj) {
                if di.id == None {
                    not_valid_set.insert(i);
                }
                if dj.id == None {
                    not_valid_set.insert(j);
                }
            }
        }

        candidates.clear();

        if !remove_list.is_empty() {
            scan_list.space.remove_indices(&mut remove_list);
            remove_list.clear()
        }

        scan_list.space.insert(LineSegment { id: i, range });
    }

    let mut result = Vec::new();

    for i in 0..dist_list.len() {
        let dist = &dist_list[i];
        if dist.id == None && !not_valid_set.contains(&i) {
            result.push(Edge::new(IntPoint::new(dist.a.x as i32, dist.a.y as i32), IntPoint::new(dist.b.x as i32, dist.b.y as i32)));
        }
    }

    result
}

fn distance(shape: &FixShape, vi: &Vector, vj: &Vector, min_length: i64, dist_list: &mut Vec<Dist>) {
    // test collinear

    if vi.index.is_neighbor(&vj.index) {
        // skip case where edges are adjacent
        return;
    }

    vert_distance(shape, vi, vj, min_length, dist_list);

    if vi.vector.dot_product(vj.vector) > 0 {
        return;
    }

    edge_distance(shape, vi, vj.a, &vj.index, min_length, dist_list);
    edge_distance(shape, vj, vi.a, &vi.index, min_length, dist_list);
}

fn vert_distance(shape: &FixShape, vi: &Vector, vj: &Vector, min_length: i64, dist_list: &mut Vec<Dist>) {
    let sqr_length = min_length * min_length;

    if (vi.a - vj.a).sqr_length() < sqr_length {
        let vi_contain = path_test(shape, &vi.index, vj.a);
        let vj_contain = path_test(shape, &vj.index, vi.a);

        if vi_contain && vj_contain {
            let dist = Dist::new(None, None, vi.a, vj.a);
            dist_list.push(dist);
        }
    }

    // other cases will be added with other pairs
}

fn edge_distance(shape: &FixShape, v: &Vector, p: FixVec, index: &ComplexIndex, min_length: i64, dist_list: &mut Vec<Dist>) {
    let e = p - v.a;

    let s = -((v.vector.cross_product(e) as f64) * v.i_scale).round() as i64;

    if 0 < s && s < min_length {
        let l = ((v.vector.dot_product(e) as f64) * v.i_scale).round();
        if l > 0.0 && l < v.length { // do not include ends, it will be added with vertDistance
            let k = l * v.i_scale;
            let qx = k * v.vector.x as f64;
            let qy = k * v.vector.y as f64;

            let q = v.a + FixVec::new(qx.round() as i64, qy.round() as i64);

            if q != v.a && q != v.b && is_corner_contain(shape, index, q) {
                let dist = Dist::new(None, Some(v.id), p, q);
                dist_list.push(dist);
            }
        }
    }
}

fn build_vectors(shape: &FixShape) -> Vec<Vector> {
    let mut vectors = Vec::new();
    let mut id = 0;
    for path_index in 0..shape.paths.len() {
        let path = &shape.paths[path_index];
        let last_index = path.len() - 1;
        let mut ai = last_index;

        for bi in 0..path.len() {
            let vector = Vector::new(path[ai], path[bi], ComplexIndex { path_index, vert_index: ai, last_index }, id);
            vectors.push(vector);
            ai = bi;
            id += 1;
        }
    }

    vectors.sort_by(|a, b| a.x_min.partial_cmp(&b.x_min).unwrap());

    vectors
}

fn build_obstacles(shape: &FixShape) -> Vec<Dist> {
    let mut obstacles = Vec::new();
    let mut id = 0;
    for path_index in 0..shape.paths.len() {
        let path = &shape.paths[path_index];
        let last_index = path.len() - 1;
        let mut ai = last_index;

        for bi in 0..path.len() {
            let obstacle = Dist::new(Some(id), None, path[ai], path[bi]);

            obstacles.push(obstacle);
            ai = bi;
            id += 1;
        }
    }

    obstacles
}

fn path_test(shape: &FixShape, index: &ComplexIndex, p: FixVec) -> bool {
    let path = &shape.paths[index.path_index];
    let b = path[index.vert_index];
    let a: FixVec;
    let c: FixVec;
    if index.is_last() {
        a = path[index.vert_index - 1];
        c = path[0]
    } else if index.is_first() {
        a = path[index.last_index];
        c = path[index.vert_index + 1];
    } else {
        a = path[index.vert_index - 1];
        c = path[index.vert_index + 1];
    }

    let m = b + b - p;

    let is_contain_p = is_contain_counter_clock_wise(a, b, c, p);
    let is_contain_m = is_contain_counter_clock_wise(a, b, c, m);

    is_contain_p && !is_contain_m
}

fn is_corner_contain(shape: &FixShape, index: &ComplexIndex, p: FixVec) -> bool {
    let path = &shape.paths[index.path_index];
    let b = path[index.vert_index];
    let a: FixVec;
    let c: FixVec;
    if index.is_last() {
        a = path[index.vert_index - 1];
        c = path[0]
    } else if index.is_first() {
        a = path[index.last_index];
        c = path[index.vert_index + 1];
    } else {
        a = path[index.vert_index - 1];
        c = path[index.vert_index + 1];
    }

    is_contain_counter_clock_wise(a, b, c, p)
}

fn is_contain_counter_clock_wise(a: FixVec, b: FixVec, c: FixVec, p: FixVec) -> bool {
    let ab = b - a;
    let bc = c - b;

    let cross = ab.cross_product(bc);
    let q = p - b;

    let sa = ab.cross_product(q);
    let sb = bc.cross_product(q);

    if cross > 0 {
        sa < 0 || sb < 0
    } else {
        sa < 0 && sb < 0 || sa < 0 && sb < 0
    }
}