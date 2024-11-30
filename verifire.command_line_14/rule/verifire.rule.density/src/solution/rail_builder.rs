use std::cmp::Ordering;
use foundation::i_overlay::i_float::point::IntPoint;
use foundation::i_overlay::i_shape::int::shape::IntShapes;
use foundation::i_tree::tree::Tree;
use crate::solution::rail::{BitSegment, Rail};
use crate::solution::sampling::Sampling;
use crate::solution::segment::ScanSegment;

pub(super) struct RailBuilder;


impl RailBuilder {
    pub(super) fn build(shapes: IntShapes, sampling: &Sampling) -> Vec<Rail> {
        let mut a_sorted = Self::segments(shapes);

        a_sorted.sort_by(|s0, s1| if s0.a.x < s1.a.x {
            Ordering::Less
        } else {
            Ordering::Greater
        });

        let mut b_sorted = a_sorted.clone();
        b_sorted.sort_by(|s0, s1| if s0.b.x < s1.b.x {
            Ordering::Less
        } else {
            Ordering::Greater
        });

        let mut rails = Vec::with_capacity(sampling.x_metric.count);

        let mut x0 = i32::MIN;
        let mut x = sampling.origin.x + sampling.x_metric.offset();
        let mut ai = 0;
        let mut bi = 0;

        let capacity = (a_sorted.len() as f64).sqrt() as usize;
        let mut tree = Tree::new(ScanSegment { a: IntPoint::ZERO, b: IntPoint::ZERO }, capacity);

        let mut ordered_segments = Vec::with_capacity(32);
        let mut stack = Vec::with_capacity(8);
        let dx = sampling.x_metric.step();

        for _ in 0..sampling.x_metric.count {

            // remove old segments
            while bi < b_sorted.len() && b_sorted[bi].b.x <= x {
                let segment = &b_sorted[bi];

                if segment.a.x <= x0 {
                    tree.delete_if_exist(segment);
                }

                bi += 1
            }

            // add new segments
            while ai < a_sorted.len() && a_sorted[ai].a.x <= x {
                let segment = &a_sorted[ai];

                if segment.b.x > x {
                    tree.insert(segment.clone());
                }

                ai += 1
            }

            if !tree.is_empty() {
                tree.fill_ordered_list_with_stack(&mut ordered_segments, &mut stack);
                let rail = Rail::with_segments(
                    &ordered_segments,
                    sampling.y_metric.log2,
                    sampling.origin.y,
                    x,
                );
                rails.push(rail);
            } else {
                rails.push(Rail::empty());
            }

            x0 = x;
            x += dx;
        }

        rails
    }

    fn segments(shapes: IntShapes) -> Vec<ScanSegment> {
        let mut capacity = 0;
        for shape in shapes.iter() {
            for path in shape.iter() {
                let mut p0 = path[path.len() - 1];
                for &pi in path.iter() {
                    if p0.x != pi.x {
                        capacity += 1;
                    }
                    p0 = pi;
                }
            }
        }

        let mut segments = Vec::with_capacity(capacity);

        for shape in shapes.iter() {
            for path in shape.iter() {
                let mut a = path[path.len() - 1];
                for &b in path.iter() {
                    if a.x != b.x {
                        let s = if a.x < b.x {
                            ScanSegment { a, b }
                        } else {
                            ScanSegment { a: b, b: a }
                        };
                        segments.push(s);
                    }
                    a = b;
                }
            }
        }

        segments
    }
}

impl Rail {
    fn with_segments(segments: &Vec<ScanSegment>, s: usize, oy: i32, x: i32) -> Self {
        if segments.is_empty() {
            return Rail::empty();
        }

        debug_assert!(segments.len() % 2 == 0);

        let mut bit_segments = Vec::with_capacity(segments.len() / 2);

        let mut i = 0;

        if s == 0 {
            while i < segments.len() {
                let s0 = &segments[i];
                let s1 = &segments[i + 1];

                let y0 = (Self::segment_y(s0, x) - oy) as u16;
                let y1 = (Self::segment_y(s1, x) - oy) as u16;

                debug_assert!(y0 <= y1);

                if y0 < y1 {
                    bit_segments.push(BitSegment::new(y0, y1));
                }

                i += 2;
            }
        } else {
            while i < segments.len() {
                let s0 = &segments[i];
                let s1 = &segments[i + 1];

                let yx0 = Self::segment_y(s0, x);
                let yx1 = Self::segment_y(s1, x);

                let y0 = ((yx0 - oy) >> s) as u16;
                let y1 = ((yx1 - oy) >> s) as u16;

                if let Some(last) = bit_segments.last_mut() {
                    if last.end + 1 == y0 {
                        last.end = y1 - 1;
                    } else if y0 < y1 {
                        bit_segments.push(BitSegment::new(y0, y1));
                    }
                } else if y0 < y1 {
                    bit_segments.push(BitSegment::new(y0, y1));
                }

                i += 2;
            }
        }

        Rail { bit_segments }
    }

    fn segment_y(segment: &ScanSegment, x: i32) -> i32 {
        if segment.a.x == segment.b.x {
            segment.a.y
        } else {
            let x = x as i64;
            let y0 = segment.a.y as i64;
            let x0 = segment.a.x as i64;
            let x1 = segment.b.x as i64;
            let y1 = segment.b.y as i64;

            let dx = x1 - x0;
            let dy = y1 - y0;

            if dx.abs() == dy.abs() {
                let k = dy.signum();
                let b = y0 - k * x0;

                (k * x + b) as i32
            } else {
                let idy = dy as i128;
                let idx = dx as i128;
                let ixx0 = (x - x0) as i128;

                let qy = ((idy * ixx0) / idx) as i64;
                (qy + y0) as i32
            }
        }
    }
}