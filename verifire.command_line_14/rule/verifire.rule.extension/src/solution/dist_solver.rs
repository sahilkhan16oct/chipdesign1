use std::cmp::Ordering;
use foundation::i_overlay::i_float::point::IntPoint;
use foundation::math::line_order::LineOrder;
use crate::solution::scan_tree::ScanTree;
use crate::solution::segment::ScanSegment;

pub(super) struct DistSolver;

impl DistSolver {
    pub(super) fn test_vertical_polygon_enclosure(outer: Vec<ScanSegment>, inner: Vec<ScanSegment>, value: i32) -> Vec<[IntPoint; 2]> {
        if outer.is_empty() || inner.is_empty() {
            return Vec::new();
        }

        let mut outer = outer;
        outer.sort_by(|e0, e1| if e0.a.x < e1.a.x {
            Ordering::Less
        } else {
            Ordering::Greater
        });

        let mut inner = inner;
        inner.sort_by(|e0, e1| if e0.a.x < e1.a.x {
            Ordering::Less
        } else {
            Ordering::Greater
        });

        let inv_outer = Self::segments_inverse_y(&outer);
        let inv_inner = Self::segments_inverse_y(&inner);

        let points_0 = Self::uniq_points(&outer);
        let points_1 = Self::uniq_points(&inv_inner);

        let mut res0 = Self::test_distance(points_0, inner, value);
        let mut res1 = Self::test_distance(points_1, inv_outer, value);
        for e in res1.iter_mut() {
            e[0].y = -e[0].y;
            e[1].y = -e[1].y;
        }

        let mut result = Vec::with_capacity(res0.len() + res1.len());
        result.append(&mut res0);
        result.append(&mut res1);

        if result.len() < 2 {
            result
        } else {
            Self::unique_edges(result)
        }
    }

    fn test_distance(points: Vec<IntPoint>, segments: Vec<ScanSegment>, value: i32) -> Vec<[IntPoint; 2]> {
        let mut scan_list = ScanTree::new(segments.len());

        let mut result = Vec::new();

        let mut i = 0;
        let mut j = 0;
        while j < points.len() {
            let x = points[j].x;

            while i < segments.len() {
                let segment = &segments[i];

                if segment.b.x < x {
                    i += 1;
                    continue;
                }

                if segment.a.x > x {
                    break;
                }

                scan_list.insert(segment.clone(), x);

                i += 1
            }

            while j < points.len() && x == points[j].x {
                let p = points[j];

                if let Some(target) = scan_list.find_under_and_nearest(p) {
                    let y = Self::y_pos(&target, p);
                    let d = p.y - y;
                    if d < value {
                        result.push([p, IntPoint::new(p.x, y)]);
                    }
                }

                j += 1
            }
        }

        result
    }

    fn y_pos(segment: &ScanSegment, point: IntPoint) -> i32 {
        if segment.a.y == segment.b.y {
            segment.a.y
        } else {
            let dx = segment.a.x - segment.b.x;
            let dy = segment.a.y - segment.b.y;
            if dx == dy {
                let b = segment.a.y - segment.a.x;
                point.x + b
            } else if dx == -dy {
                let b = segment.a.y + segment.a.x;
                b - point.x
            } else {
                let k = (dy as f64) / (dx as f64);
                let x0 = segment.a.x as f64;
                let y0 = segment.a.x as f64;
                let b = y0 - k * x0;
                let y = (k * x0 + b).round();

                y as i32
            }
        }
    }

    fn uniq_points(segments: &Vec<ScanSegment>) -> Vec<IntPoint> {
        let mut points = Vec::with_capacity(2 * segments.len());
        for s in segments.iter() {
            points.push(s.a);
            points.push(s.b);
        }
        points.sort_by(|a, b| a.order_by_line(*b));
        let mut result = Vec::with_capacity(points.len());
        let mut q = points[0];
        result.push(q);
        for &p in points.iter() {
            if q != p {
                result.push(p);
                q = p;
            }
        }

        result
    }

    fn segments_inverse_y(segments: &Vec<ScanSegment>) -> Vec<ScanSegment> {
        let mut result = Vec::with_capacity(segments.len());
        for s in segments.iter() {
            let a = IntPoint { x: s.a.x, y: -s.a.y };
            let b = IntPoint { x: s.b.x, y: -s.b.y };
            result.push(ScanSegment { a, b });
        }

        result
    }

    fn unique_edges(edges: Vec<[IntPoint; 2]>) -> Vec<[IntPoint; 2]> {
        let mut edges = edges;
        let mut result = Vec::with_capacity(edges.len());

        for e in edges.iter_mut() {
            if !e[0].order_by_line_compare(e[1]) {
                let p = e[0];
                e[0] = e[1];
                e[1] = p;
            }
        }

        edges.sort_by(|e0, e1| if e0[0] != e1[1] {
            e0[0].order_by_line(e1[0])
        } else {
            e0[1].order_by_line(e1[1])
        });

        let mut e = edges[0].clone();
        result.push(e);
        for ei in edges.into_iter() {
            if e[0] != ei[0] && e[1] != ei[1] {
                result.push(ei.clone());
                e = ei;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use foundation::i_overlay::i_float::point::IntPoint;
    use crate::solution::dist_solver::DistSolver;
    use crate::solution::segment::ScanSegment;

    #[test]
    fn test_0() {
        let segments = vec![
            ScanSegment { a: IntPoint { x: -4, y: 1 }, b: IntPoint { x: -1, y: 1 } },
            ScanSegment { a: IntPoint { x: -2, y: -1 }, b: IntPoint { x: 2, y: -1 } },
            ScanSegment { a: IntPoint { x: 1, y: 1 }, b: IntPoint { x: 4, y: 1 } },
        ];

        let points = vec![
            IntPoint { x: -2, y: 3 },
            IntPoint { x: 0, y: 2 },
            IntPoint { x: 2, y: 1 },
        ];

        let result = DistSolver::test_distance(points, segments, 3);

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_1() {
        let segments = vec![
            ScanSegment { a: IntPoint { x: 0, y: 1 }, b: IntPoint { x: 2, y: 1 } },
            ScanSegment { a: IntPoint { x: 0, y: 3 }, b: IntPoint { x: 4, y: 3 } },
            ScanSegment { a: IntPoint { x: 0, y: 5 }, b: IntPoint { x: 6, y: 5 } },
        ];

        let points = vec![
            IntPoint { x: 1, y: 2 },
            IntPoint { x: 1, y: 4 },
        ];

        let result = DistSolver::test_distance(points, segments, 2);

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_2() {
        let segments = vec![
            ScanSegment { a: IntPoint { x: -3, y: 3 }, b: IntPoint { x: -1, y: 1 } },
            ScanSegment { a: IntPoint { x: -1, y: 1 }, b: IntPoint { x: 1, y: 1 } },
            ScanSegment { a: IntPoint { x: 1, y: 1 }, b: IntPoint { x: 3, y: 3 } },
        ];

        let points = vec![
            IntPoint { x: -2, y: 1 },
            IntPoint { x: 0, y: 1 },
            IntPoint { x: 2, y: 1 },
        ];

        let result = DistSolver::test_distance(points, segments, 2);

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_3() {
        let segments = vec![
            ScanSegment { a: IntPoint { x: -3, y: 3 }, b: IntPoint { x: -1, y: 1 } },
            ScanSegment { a: IntPoint { x: -1, y: 1 }, b: IntPoint { x: 1, y: 1 } },
            ScanSegment { a: IntPoint { x: 1, y: 1 }, b: IntPoint { x: 3, y: 3 } },
        ];

        let points = vec![
            IntPoint { x: -2, y: 3 },
            IntPoint { x: 0, y: 3 },
            IntPoint { x: 2, y: 3 },
        ];

        let result = DistSolver::test_distance(points, segments, 2);

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_4() {
        let outer = vec![
            ScanSegment { a: IntPoint { x: -3, y: 2 }, b: IntPoint { x: -1, y: 4 } },
            ScanSegment { a: IntPoint { x: -1, y: 4 }, b: IntPoint { x: 1, y: 4 } },
            ScanSegment { a: IntPoint { x: 1, y: 4 }, b: IntPoint { x: 3, y: 2 } },
        ];

        let inner = vec![
            ScanSegment { a: IntPoint { x: -5, y: 0 }, b: IntPoint { x: 5, y: 0 } }
        ];

        assert_eq!(2, DistSolver::test_vertical_polygon_enclosure(outer.clone(), inner.clone(), 3).len());
        assert_eq!(4, DistSolver::test_vertical_polygon_enclosure(outer, inner, 5).len());
    }

    #[test]
    fn test_5() {
        let outer = vec![
            ScanSegment { a: IntPoint { x: -3, y: 3 }, b: IntPoint { x: 3, y: 3 } }
        ];

        let inner = vec![
            ScanSegment { a: IntPoint { x: -4, y: -3 }, b: IntPoint { x: -2, y: -1 } },
            ScanSegment { a: IntPoint { x: -2, y: -1 }, b: IntPoint { x: 2, y: -1 } },
            ScanSegment { a: IntPoint { x: 2, y: -1 }, b: IntPoint { x: 4, y: -3 } },
        ];

        assert_eq!(0, DistSolver::test_vertical_polygon_enclosure(outer.clone(), inner.clone(), 4).len());
        assert_eq!(2, DistSolver::test_vertical_polygon_enclosure(outer.clone(), inner.clone(), 5).len());
        assert_eq!(4, DistSolver::test_vertical_polygon_enclosure(outer, inner, 6).len());
    }
}