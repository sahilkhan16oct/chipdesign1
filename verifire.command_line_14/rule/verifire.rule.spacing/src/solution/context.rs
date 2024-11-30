use std::collections::HashSet;
use foundation::i_overlay::i_float::fix_vec::FixVec;
use foundation::i_overlay::i_float::point::IntPoint;
use foundation::space_extension::square_index::SquareIndex;
use foundation::space_extension::square_range::SquareRange;
use foundation::space_extension::square_space::{SquareSpace, SquareSpaceBuffer};
use foundation::space_extension::square_vector::SquareVector;
use foundation::i_overlay::line_range::LineRange;
use foundation::task::direction::SpaceDirection;
use crate::solution::dist::Dist;
use crate::solution::edge::ErrorEdge;
use crate::solution::space::Space;

pub struct SpaceContext<S: Space> {
    space: S,
    min_length: i64,
    direction: SpaceDirection,
}

impl<S: Space> SpaceContext<S> {
    pub fn new(space: S, min_length: i64, direction: SpaceDirection) -> Self {
        Self { space, min_length, direction }
    }

    pub fn find_problems(&self) -> Vec<[IntPoint; 2]> {
        let vectors = self.space.vectors();

        let sq_space = SquareSpace::new(&vectors);

        let mut ids = Vec::new();
        let mut dist_list = Vec::new();
        let mut edges = HashSet::new();
        let mut space_buffer = SquareSpaceBuffer::new();

        for row in sq_space.rows.iter() {
            if let Some(row) = row {
                for heap in row.heaps.iter() {
                    if let Some(heap) = heap {
                        for segment in heap.iter() {
                            let vi = &vectors[segment.id];
                            let range = vi.delta_range(self.min_length as i32, sq_space.indexer.range);

                            ids.clear();
                            sq_space.ids_in_range(&range, segment.id, &mut space_buffer, &mut ids);

                            if !ids.is_empty() {
                                for &id in ids.iter() {
                                    if id != segment.id {
                                        let vj = &vectors[id];
                                        if self.space.are_colliding(&vj.index, &vi.index) {
                                            self.distance(vi, vj, &mut dist_list);
                                        }
                                    }
                                }

                                if dist_list.is_empty() {
                                    continue;
                                }

                                for dist in dist_list.iter() {
                                    let mut is_crossed = false;
                                    for &id in ids.iter() {
                                        let v = &vectors[id];
                                        if dist.is_cross(v) {
                                            is_crossed = true;
                                            break;
                                        }
                                    }

                                    if !is_crossed {
                                        edges.insert(ErrorEdge::new(dist.a, dist.b));
                                    }
                                }

                                dist_list.clear();
                            }
                        }
                    }
                }
            }
        }

        edges.into_iter()
            .map(|edge| [IntPoint::new(edge.a.x as i32, edge.a.y as i32), IntPoint::new(edge.b.x as i32, edge.b.y as i32)]) // Convert each ErrorEdge to [FixVec; 2]
            .collect()
    }

    fn distance(&self, vi: &SquareVector, vj: &SquareVector, dist_list: &mut Vec<Dist>) {
        if vi.a == vj.a || vi.a == vj.b || vi.b == vj.a || vi.b == vj.b {
            return;
        }
        self.verts_distance(&vi, &vj, self.min_length, self.direction, dist_list);
        self.edge_distance(&vi, &vj, self.min_length, self.direction, dist_list);
    }


    fn verts_distance(&self, vi: &SquareVector, vj: &SquareVector, min_length: i64, direction: SpaceDirection, dist_list: &mut Vec<Dist>) {
        let sqr_length = min_length * min_length;

        // a - a
        if let Some(dist) = self.vert_distance(&vi.index, vi.a, &vj.index, vj.a, sqr_length, direction) {
            dist_list.push(dist)
        }

        // b - a
        if let Some(dist) = self.vert_distance(&vi.index.next_index(), vi.b, &vj.index, vj.a, sqr_length, direction) {
            dist_list.push(dist)
        }
    }

    fn vert_distance(&self, i0: &SquareIndex, p0: FixVec, i1: &SquareIndex, p1: FixVec, sqr_length: i64, direction: SpaceDirection) -> Option<Dist> {
        if (p0 - p1).sqr_length() < sqr_length {
            let vi_contain = self.space.path_test(i0, p1);
            let vj_contain = self.space.path_test(i1, p0);

            if vi_contain && vj_contain {
                let dist = Dist::new(p0, p1);
                if dist.validate(direction) {
                    return Some(dist);
                }
            }
        }

        None
    }


    fn edge_distance(&self, vi: &SquareVector, vj: &SquareVector, min_length: i64, direction: SpaceDirection, dist_list: &mut Vec<Dist>) {
        let dv = vi.vector();

        if dv.x == 0 && direction != SpaceDirection::Y {
            self.edge_x(vi, vj, min_length, dist_list);
            return;
        }

        if dv.y == 0 && direction != SpaceDirection::X {
            self.edge_y(vi, vj, min_length, dist_list);
            return;
        }

        if dv.x.abs() == dv.y.abs() {
            self.edge_xy(vi, vj, min_length, dist_list);
            return;
        }

        self.edge_d(vi, vj, min_length, dist_list);
    }


    fn edge_y(&self, vi: &SquareVector, vj: &SquareVector, min_length: i64, dist_list: &mut Vec<Dist>) {
        let p = vj.a;
        let dy = p.y - vi.a.y;
        let x = p.x as i32;
        if dy.abs() < min_length && vi.range.x_min < x && x < vi.range.x_max {
            if vi.a.x < vi.b.x && dy < 0 || vi.a.x > vi.b.x && dy > 0 {
                return;
            }

            let pn = FixVec::new(p.x, vi.a.y);

            if self.space.is_contain_clock_wise(&vj.index, pn) {
                dist_list.push(Dist::new(p, pn));
            }
        }
    }

    fn edge_x(&self, vi: &SquareVector, vj: &SquareVector, min_length: i64, dist_list: &mut Vec<Dist>) {
        let p = vj.a;
        let dx = p.x - vi.a.x;
        let y = p.y as i32;
        if dx.abs() < min_length && vi.range.y_min < y && y < vi.range.y_max {
            if vi.a.y < vi.b.y && dx > 0 || vi.a.y > vi.b.y && dx < 0 {
                return;
            }

            let pn = FixVec::new(vi.a.x, p.y);

            if self.space.is_contain_clock_wise(&vj.index, pn) {
                dist_list.push(Dist::new(p, pn));
            }
        }
    }

    fn edge_xy(&self, vi: &SquareVector, vj: &SquareVector, min_length: i64, dist_list: &mut Vec<Dist>) {
        let p = vj.a;
        let e = p - vi.a;
        let sqrt2_n = vi.normalize_for_diagonal();
        let cross = sqrt2_n.cross_product(e);

        let sqr_dist = (cross * cross) / 2;
        let min_sqr_distance = min_length * min_length;
        if cross > 0 && sqr_dist < min_sqr_distance {
            let proj = sqrt2_n.dot_product(e);
            let s = sqrt2_n * proj;
            let pn = vi.a + FixVec::new(s.x / 2, s.y / 2);

            if self.space.is_contain_clock_wise(&vj.index, pn) && vi.is_line_point_inside(pn) {
                dist_list.push(Dist::new(p, pn));
            }
        }
    }

    fn edge_d(&self, vi: &SquareVector, vj: &SquareVector, min_length: i64, dist_list: &mut Vec<Dist>) {
        let p = vj.a;
        let ie = p - vi.a;
        let n = vi.normalize();
        let e = ie.like_f64vec();
        let cross = n.cross_product(e);


        if cross > 0.0 && cross < (min_length as f64) {
            let proj = n.dot_product(e);
            let s = n * proj;
            let x = s.x.round() as i64;
            let y = s.y.round() as i64;

            let pn = vi.a + FixVec::new(x, y);

            if self.space.is_contain_clock_wise(&vj.index, pn) && vi.is_line_point_inside(pn) {
                dist_list.push(Dist::new(p, pn));
            }
        }
    }
}

trait VecExt {
    fn is_line_point_inside(&self, p: FixVec) -> bool;

    fn delta_range(&self, delta: i32, y_clamp: LineRange) -> SquareRange;
}

impl VecExt for SquareVector {
    fn is_line_point_inside(&self, p: FixVec) -> bool {
        p != self.a && p != self.b && self.range.is_contain(p)
    }

    fn delta_range(&self, delta: i32, y_clamp: LineRange) -> SquareRange {
        let x_min: i32;
        let x_max: i32;
        let y_min: i32;
        let y_max: i32;

        if self.a.x == self.b.x {
            if self.b.y > self.a.y {
                x_min = self.range.x_min - delta;
                x_max = self.range.x_max;
            } else {
                x_min = self.range.x_min;
                x_max = self.range.x_max + delta;
            }

            y_min = (self.range.y_min - delta).clamp(y_clamp.min, y_clamp.max);
            y_max = (self.range.y_max + delta).clamp(y_clamp.min, y_clamp.max);
        } else if self.a.y == self.b.y {
            if self.b.x > self.a.x {
                y_min = self.range.y_min;
                y_max = (self.range.y_max + delta).clamp(y_clamp.min, y_clamp.max);
            } else {
                y_min = (self.range.y_min - delta).clamp(y_clamp.min, y_clamp.max);
                y_max = self.range.y_max;
            }

            x_min = self.range.x_min - delta;
            x_max = self.range.x_max + delta;
        } else {
            x_min = self.range.x_min - delta;
            x_max = self.range.x_max + delta;
            y_min = (self.range.y_min - delta).clamp(y_clamp.min, y_clamp.max);
            y_max = (self.range.y_max + delta).clamp(y_clamp.min, y_clamp.max);
        }

        SquareRange::new(x_min, y_min, x_max, y_max)
    }
}
