use std::collections::HashMap;
use foundation::i_overlay::bind::segment::IdSegment;
use foundation::i_overlay::i_shape::int::path::IntPath;
use foundation::i_overlay::i_shape::int::shape::{IntShape, IntShapes};
use foundation::i_overlay::id_point::IdPoint;
use foundation::i_overlay::x_segment::XSegment;
use crate::solution::scan_hole::ScanHoleTree;

pub(super) struct Inverter;

struct ContourSegment {
    id: usize,
    is_top: bool,
    x_segment: XSegment,
}

impl ContourSegment {
    fn id_segment(&self) -> IdSegment {
        IdSegment { id: self.id, x_segment: self.x_segment }
    }
}

impl Inverter {
    pub(super) fn invert(shapes: IntShapes) -> IntShapes {
        let mut positive_paths = Vec::with_capacity(shapes.len() / 2);
        let mut negative_paths = Vec::with_capacity(shapes.len() / 2);

        let mut i_points = Vec::with_capacity(negative_paths.len());
        let mut segments = Vec::with_capacity(4 * positive_paths.len());
        for shape in shapes.into_iter() {
            let mut is_first = true;
            for mut path in shape.into_iter() {
                path.reverse();
                if is_first {
                    i_points.push(IdPoint::new(negative_paths.len(), path[0]));
                    negative_paths.push(path);
                    is_first = false;
                } else {
                    let mut path_segments = Self::id_segments(positive_paths.len(), &path);
                    segments.append(&mut path_segments);
                    positive_paths.push(path);
                }
            }
        }

        i_points.sort_by(|a, b| a.point.x.cmp(&b.point.x));
        segments.sort_by(|a, b| a.x_segment.a.x.cmp(&b.x_segment.b.x));

        let (parent_for_child, children_count_for_parent) = Self::private_solve(i_points, segments);

        let mut result = Vec::with_capacity(positive_paths.len());
        for path in positive_paths.into_iter() {
            let id = result.len();
            let shape = if let Some(count) = children_count_for_parent.get(&id) {
                let mut shape = IntShape::with_capacity(1 + count);
                shape.push(path);
                shape
            } else {
                vec![path]
            };
            result.push(shape);
        }

        for (&child, parent) in parent_for_child.iter() {
            let shape = result.get_mut(*parent).unwrap();
            shape.push(negative_paths[child].clone())
        }

        result
    }

    fn private_solve(i_points: Vec<IdPoint>, segments: Vec<ContourSegment>) -> (HashMap<usize, usize>, HashMap<usize, usize>) {
        let mut top_scan = ScanHoleTree::new(segments.len() / 2);
        let mut bottom_scan = ScanHoleTree::new(segments.len() / 2);

        let mut parent_for_child: HashMap<usize, usize> = HashMap::new();
        let mut children_count_for_parent: HashMap<usize, usize> = HashMap::new();

        let mut i = 0;
        let mut j = 0;

        while i < i_points.len() {
            let x = i_points[i].point.x;

            while j < segments.len() && segments[j].x_segment.a.x <= x {
                let segment = &segments[j];
                if segment.x_segment.b.x > x {
                    if segment.is_top {
                        top_scan.insert(segment.id_segment(), x);
                    } else {
                        bottom_scan.insert(segment.id_segment(), x);
                    }
                }
                j += 1
            }

            while i < i_points.len() && i_points[i].point.x == x {
                let p = i_points[i].point;

                let i0 = bottom_scan.find_under_and_nearest(p, x);
                let i1 = top_scan.find_above_and_nearest(p, x);
                if i0 == i1 && i0 < usize::MAX {
                    let child_index = i_points[i].id;
                    parent_for_child.insert(child_index, i0);
                    if let Some(children_count) = children_count_for_parent.get_mut(&child_index) {
                        *children_count += 1;
                    }
                }

                i += 1;
            }
        }

        (parent_for_child, children_count_for_parent)
    }

    fn id_segments(id: usize, path: &IntPath) -> Vec<ContourSegment> {
        let n = path.len();
        let mut segments = Vec::with_capacity(n);

        let mut b = path[n - 1];
        for &a in path.iter() {
            if a.x != b.x {
                let is_top = a.x > b.x;
                let x_segment = if is_top {
                    XSegment::new(b, a)
                } else {
                    XSegment::new(a, b)
                };
                segments.push(ContourSegment { id, is_top, x_segment });
            }
            b = a
        }
        segments
    }
}

#[cfg(test)]
mod tests {
    use foundation::i_overlay::i_float::point::IntPoint;
    use crate::solution::inverter::Inverter;

    #[test]
    fn test_0() {
        let shapes = [
            [
                [
                    IntPoint::new(-10, -10),
                    IntPoint::new(-10, 10),
                    IntPoint::new(10, 10),
                    IntPoint::new(10, -10)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let result = Inverter::invert(shapes);

        assert_eq!(result.is_empty(), true);
    }

    #[test]
    fn test_1() {
        let shapes = [
            [
                [
                    IntPoint::new(-10, -10),
                    IntPoint::new(-10, 10),
                    IntPoint::new(10, 10),
                    IntPoint::new(10, -10)
                ].to_vec(),
                [
                    IntPoint::new(-5, -5),
                    IntPoint::new(5, -5),
                    IntPoint::new(5, 5),
                    IntPoint::new(-5, 5)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let result = Inverter::invert(shapes);

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_2() {
        let shapes = [
            [
                [
                    IntPoint::new(-10, -10),
                    IntPoint::new(-10, 10),
                    IntPoint::new(10, 10),
                    IntPoint::new(10, -10)
                ].to_vec(),
                [
                    IntPoint::new(-5, -5),
                    IntPoint::new(5, -5),
                    IntPoint::new(5, 5),
                    IntPoint::new(-5, 5)
                ].to_vec(),
            ].to_vec(),
            [
                [
                    IntPoint::new(-20, -20),
                    IntPoint::new(-20, 20),
                    IntPoint::new(20, 20),
                    IntPoint::new(20, -20)
                ].to_vec(),
                [
                    IntPoint::new(-15, -15),
                    IntPoint::new(15, -15),
                    IntPoint::new(15, 15),
                    IntPoint::new(-15, 15)
                ].to_vec(),
            ].to_vec()
        ].to_vec();

        let result = Inverter::invert(shapes);

        assert_eq!(result.len(), 2);

        let paths_count = result.iter().fold(0, |acc, shape| acc + shape.len());

        assert_eq!(paths_count, 3);
    }

    #[test]
    fn test_3() {
        let shapes = [
            [
                [
                    IntPoint::new(-10, -10),
                    IntPoint::new(-10, 10),
                    IntPoint::new(10, 10),
                    IntPoint::new(10, -10)
                ].to_vec(),
                [
                    IntPoint::new(-5, -5),
                    IntPoint::new(5, -5),
                    IntPoint::new(5, 5),
                    IntPoint::new(-5, 5)
                ].to_vec(),
            ].to_vec(),
            [
                [
                    IntPoint::new(-10, 10),
                    IntPoint::new(-10, 30),
                    IntPoint::new(10, 30),
                    IntPoint::new(10, 10)
                ].to_vec(),
                [
                    IntPoint::new(-5, 15),
                    IntPoint::new(5, 15),
                    IntPoint::new(5, 25),
                    IntPoint::new(-5, 25)
                ].to_vec(),
            ].to_vec()
        ].to_vec();

        let result = Inverter::invert(shapes);

        assert_eq!(result.len(), 2);

        let paths_count = result.iter().fold(0, |acc, shape| acc + shape.len());

        assert_eq!(paths_count, 2);
    }

    #[test]
    fn test_4() {
        let shapes = [
            [
                [
                    IntPoint::new(-10, -10),
                    IntPoint::new(-10, 10),
                    IntPoint::new(10, 10),
                    IntPoint::new(10, -10)
                ].to_vec(),
                [
                    IntPoint::new(-5, -5),
                    IntPoint::new(5, -5),
                    IntPoint::new(5, 5),
                    IntPoint::new(-5, 5)
                ].to_vec(),
            ].to_vec(),
            [
                [
                    IntPoint::new(10, -10),
                    IntPoint::new(10, 10),
                    IntPoint::new(30, 10),
                    IntPoint::new(30, -10)
                ].to_vec(),
                [
                    IntPoint::new(15, -5),
                    IntPoint::new(25, -5),
                    IntPoint::new(25, 5),
                    IntPoint::new(15, 5)
                ].to_vec(),
            ].to_vec(),
        ].to_vec();

        let result = Inverter::invert(shapes);

        assert_eq!(result.len(), 2);

        let paths_count = result.iter().fold(0, |acc, shape| acc + shape.len());

        assert_eq!(paths_count, 2);
    }

    #[test]
    fn test_5() {
        let shapes = [
            [
                [
                    IntPoint::new(-10, -10),
                    IntPoint::new(-10, 10),
                    IntPoint::new(10, 10),
                    IntPoint::new(10, -10)
                ].to_vec(),
                [
                    IntPoint::new(-5, -5),
                    IntPoint::new(5, -5),
                    IntPoint::new(5, 5),
                    IntPoint::new(-5, 5)
                ].to_vec(),
            ].to_vec(),
            [
                [
                    IntPoint::new(-20, -20),
                    IntPoint::new(-20, 20),
                    IntPoint::new(20, 20),
                    IntPoint::new(20, -20)
                ].to_vec(),
                [
                    IntPoint::new(-15, -15),
                    IntPoint::new(15, -15),
                    IntPoint::new(15, 15),
                    IntPoint::new(-15, 15)
                ].to_vec(),
            ].to_vec(),
            [
                [
                    IntPoint::new(-30, -30),
                    IntPoint::new(-30, 30),
                    IntPoint::new(30, 30),
                    IntPoint::new(30, -30)
                ].to_vec(),
                [
                    IntPoint::new(-25, -25),
                    IntPoint::new(25, -25),
                    IntPoint::new(25, 25),
                    IntPoint::new(-25, 25)
                ].to_vec(),
            ].to_vec()
        ].to_vec();

        let result = Inverter::invert(shapes);

        assert_eq!(result.len(), 3);

        let paths_count = result.iter().fold(0, |acc, shape| acc + shape.len());

        assert_eq!(paths_count, 5);
    }

    #[test]
    fn test_6() {
        let shapes = [
            [
                [
                    IntPoint::new(-10, -10),
                    IntPoint::new(-10, 10),
                    IntPoint::new(10, 10),
                    IntPoint::new(10, -10)
                ].to_vec(),
            ].to_vec(),
            [
                [
                    IntPoint::new(-20, -20),
                    IntPoint::new(-20, 20),
                    IntPoint::new(20, 20),
                    IntPoint::new(20, -20)
                ].to_vec(),
                [
                    IntPoint::new(-15, -15),
                    IntPoint::new(15, -15),
                    IntPoint::new(15, 15),
                    IntPoint::new(-15, 15)
                ].to_vec(),
            ].to_vec(),
        ].to_vec();

        let result = Inverter::invert(shapes);

        assert_eq!(result.len(), 1);

        let paths_count = result.iter().fold(0, |acc, shape| acc + shape.len());

        assert_eq!(paths_count, 2);
    }

    #[test]
    fn test_7() {
        let shapes = [
            [
                [
                    IntPoint::new(0, 0),
                    IntPoint::new(0, 25),
                    IntPoint::new(45, 25),
                    IntPoint::new(45, 0)
                ].to_vec(),
                [
                    IntPoint::new(15, 5),
                    IntPoint::new(15, 10),
                    IntPoint::new(25, 10),
                    IntPoint::new(25, 5),
                    IntPoint::new(40, 5),
                    IntPoint::new(40, 20),
                    IntPoint::new(5, 20),
                    IntPoint::new(5, 5)
                ].to_vec()
            ].to_vec(),
            [
                [
                    IntPoint::new(30, 10),
                    IntPoint::new(30, 15),
                    IntPoint::new(35, 15),
                    IntPoint::new(35, 10)
                ].to_vec(),
            ].to_vec(),
        ].to_vec();

        let result = Inverter::invert(shapes);

        assert_eq!(result.len(), 1);

        let paths_count = result.iter().fold(0, |acc, shape| acc + shape.len());

        assert_eq!(paths_count, 2);
    }
}