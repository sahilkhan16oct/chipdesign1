use crate::algorithm::slice::scan::{ScanSegment, ScanTree};
use crate::algorithm::slice::slice_type::SliceType;
use crate::i_overlay::i_float::point::IntPoint;
use crate::i_overlay::i_float::triangle::Triangle;
use crate::i_overlay::i_shape::int::path::IntPath;
use crate::i_overlay::i_shape::int::shape::{IntShape, PointsCount};
use crate::i_overlay::x_segment::XSegment;
use crate::i_tree::node::EMPTY_REF;

pub trait Slice {
    fn vertical_slice(&self, slice_type: SliceType) -> Vec<IntPath>;
}

impl Slice for IntShape {
    fn vertical_slice(&self, slice_type: SliceType) -> Vec<IntPath> {
        let segments = Internal::shape_to_segments(self);
        let anchors = Internal::shape_to_anchors(self);
        let mut result = Vec::with_capacity(segments.len() / 2);

        let mut tree = ScanTree::new(32);
        let mut pieces = Vec::new();
        let mut to_remove = Vec::new();

        let mut sx = if segments.is_empty() {
            i32::MAX
        } else {
            segments[0].a.x
        };

        let mut i = 0;
        let mut j = 0;

        while i < segments.len() {
            while i < segments.len() && sx == segments[i].a.x {
                // add new segments
                let s = &segments[i];
                tree.insert(s.clone());

                i += 1
            }

            sx = if i < segments.len() {
                segments[i].a.x
            } else {
                i32::MAX
            };

            while j < anchors.len() && anchors[j].point().x <= sx {
                let ax = anchors[j].point().x;
                while j < anchors.len() && anchors[j].point().x <= ax {
                    let anchor = &anchors[j];

                    match anchor {
                        Anchor::TopSegment(s) => {
                            if let Some(path) = Internal::top_segment(&mut tree, *s, &mut to_remove, &mut pieces) {
                                result.push(Internal::rect_path(path, slice_type));
                            }
                        }
                        Anchor::TopPoint(p) => {
                            if let Some(path) = Internal::top_point(&mut tree, *p, &mut to_remove, &mut pieces) {
                                result.push(Internal::rect_path(path, slice_type));
                            }
                        }

                        Anchor::BottomSegment(s) => {
                            if let Some(path) = Internal::bottom_segment(&mut tree, *s, &mut to_remove, &mut pieces) {
                                result.push(Internal::rect_path(path, slice_type));
                            }
                        }
                        Anchor::BottomPoint(p) => {
                            if let Some(path) = Internal::bottom_point(&mut tree, *p, &mut to_remove, &mut pieces) {
                                result.push(Internal::rect_path(path, slice_type));
                            }
                        }
                    }
                    j += 1
                }

                if !pieces.is_empty() {
                    for s in to_remove.iter() {
                        tree.tree.delete(&ScanSegment { x_segment: s.clone(), is_visited: true });
                    }

                    to_remove.clear();

                    for s in pieces.iter() {
                        tree.insert(s.clone());
                    }

                    pieces.clear();
                }
            }
        }

        result
    }
}


enum Anchor {
    TopPoint(IntPoint),
    BottomPoint(IntPoint),
    TopSegment(XSegment),
    BottomSegment(XSegment),
}

impl Anchor {
    fn point(&self) -> IntPoint {
        match self {
            Anchor::TopPoint(p) => {
                *p
            }
            Anchor::BottomPoint(p) => {
                *p
            }
            Anchor::TopSegment(s) => {
                s.b
            }
            Anchor::BottomSegment(s) => {
                s.b
            }
        }
    }
}

struct Internal;

impl Internal {
    fn tail(x_segment: &XSegment, x: i32) -> XSegment {
        let y = if x_segment.a.y == x_segment.b.y {
            x_segment.a.y
        } else {
            let y0 = x_segment.a.y as i64;
            let x0 = x_segment.a.x as i64;
            let y1 = x_segment.b.y as i64;
            let k = (y1 - y0).signum();
            let b = y0 - k * x0;

            (k * (x as i64) + b) as i32
        };

        XSegment { a: IntPoint { x, y }, b: x_segment.b }
    }

    fn rect_path(path: Vec<IntPoint>, slice_type: SliceType) -> IntPath {
        match slice_type {
            SliceType::Original => {
                path
            }
            SliceType::OuterBox => {
                let mut result = Vec::with_capacity(4);
                let p0 = path[0];
                let mut min_x = p0.x;
                let mut max_x = min_x;
                let mut min_y = p0.y;
                let mut max_y = min_y;
                for i in 1..path.len() {
                    let p = path[i];
                    min_x = min_x.min(p.x);
                    max_x = max_x.max(p.x);
                    min_y = min_y.min(p.y);
                    max_y = max_y.max(p.y);
                }

                result.push(IntPoint::new(min_x, min_y));
                result.push(IntPoint::new(min_x, max_y));
                result.push(IntPoint::new(max_x, max_y));
                result.push(IntPoint::new(max_x, min_y));

                result
            }
        }
    }

    fn shape_to_segments(shape: &IntShape) -> Vec<XSegment> {
        let n = shape.points_count();
        let mut segments = Vec::with_capacity(n);

        for i in 0..shape.len() {
            let path = &shape[i];

            let mut a = path[path.len() - 1];
            for &p in path.iter() {
                let b = p;
                if b.x != a.x {
                    let segment = if a.x < b.x {
                        XSegment { a, b }
                    } else {
                        XSegment { a: b, b: a }
                    };
                    segments.push(segment);
                }
                a = b;
            }
        }
        segments.sort_by(|p0, p1| p0.a.x.cmp(&p1.a.x));

        segments
    }

    fn shape_to_anchors(shape: &IntShape) -> Vec<Anchor> {
        let n = shape.points_count();
        let mut anchors = Vec::with_capacity(n);

        for path in shape.iter() {
            let n = path.len();
            let mut b = path[n - 2];
            let mut a = path[n - 1];
            for i in 0..path.len() {
                let c = b;
                b = a;
                a = path[i];

                if a.x >= b.x && c.x >= b.x {
                    if !Triangle::is_clockwise_point(a, c, b) {
                        // split
                        if a.x > b.x {
                            anchors.push(Anchor::BottomPoint(b));
                        } else if c.x > b.x {
                            anchors.push(Anchor::TopPoint(b));
                        }
                    }
                    continue;
                }

                if a.x <= b.x && c.x <= b.x {
                    if Triangle::is_clockwise_point(a, c, b) {
                        // end

                        if a.x < b.x {
                            anchors.push(Anchor::TopSegment(XSegment::new(a, b)));
                        }
                    } else {
                        // merge

                        if a.x < b.x {
                            anchors.push(Anchor::TopSegment(XSegment::new(a, b)));
                        }

                        if c.x < b.x {
                            anchors.push(Anchor::BottomSegment(XSegment::new(c, b)));
                        }
                    }

                    continue;
                }


                if a.x > b.x && b.x > c.x {
                    anchors.push(Anchor::BottomSegment(XSegment::new(c, b)));
                    continue;
                }

                if a.x < b.x && b.x < c.x {
                    anchors.push(Anchor::TopSegment(XSegment::new(a, b)));
                    continue;
                }
            }
        }
        anchors.sort_by(|a, b| a.point().x.cmp(&b.point().x));

        anchors
    }

    fn top_segment(tree: &mut ScanTree, target: XSegment, to_remove: &mut Vec<XSegment>, pieces: &mut Vec<XSegment>) -> Option<IntPath> {
        let i0 = tree.find_segment(target);
        Self::top(tree, target.b.x, i0, to_remove, pieces)
    }

    fn bottom_point(tree: &mut ScanTree, target: IntPoint, to_remove: &mut Vec<XSegment>, pieces: &mut Vec<XSegment>) -> Option<IntPath> {
        let i0 = tree.find_under_point(target);
        Self::top(tree, target.x, i0, to_remove, pieces)
    }

    fn top_point(tree: &mut ScanTree, target: IntPoint, to_remove: &mut Vec<XSegment>, pieces: &mut Vec<XSegment>) -> Option<IntPath> {
        let i0 = tree.find_above_point(target);
        Self::bottom(tree, target.x, i0, to_remove, pieces)
    }

    fn bottom_segment(tree: &mut ScanTree, target: XSegment, to_remove: &mut Vec<XSegment>, pieces: &mut Vec<XSegment>) -> Option<IntPath> {
        let i0 = tree.find_segment(target);
        Self::bottom(tree, target.b.x, i0, to_remove, pieces)
    }

    fn bottom(tree: &mut ScanTree, line_x: i32, i0: u32, to_remove: &mut Vec<XSegment>, pieces: &mut Vec<XSegment>) -> Option<IntPath> {
        if i0 == EMPTY_REF {
            return None;
        }
        let s0 = tree.segment(i0);
        let i1 = tree.find_under_segment(s0);
        if i0 == EMPTY_REF {
            return None;
        }
        let s1 = tree.segment(i1);

        tree.make_visited(i0);
        tree.make_visited(i1);

        let a0 = s0.a;
        let b0 = Internal::cut_tail(s0, line_x, to_remove, pieces);

        let a1 = s1.a;
        let b1 = Internal::cut_tail(s1, line_x, to_remove, pieces);

        let n = if a0 != a1 && b0 != b1 { 4 } else { 3 };

        let mut path = Vec::with_capacity(n);

        path.push(a1);
        if a1 != a0 {
            path.push(a0);
        }
        path.push(b0);

        if b0 != b1 {
            path.push(b1);
        }

        Some(path)
    }

    fn top(tree: &mut ScanTree, line_x: i32, i0: u32, to_remove: &mut Vec<XSegment>, pieces: &mut Vec<XSegment>) -> Option<IntPath> {
        if i0 == EMPTY_REF {
            return None;
        }
        let s0 = tree.segment(i0);
        let i1 = tree.find_above_segment(s0);
        if i0 == EMPTY_REF {
            return None;
        }
        let s1 = tree.segment(i1);

        tree.make_visited(i0);
        tree.make_visited(i1);

        let a0 = s0.a;
        let b0 = Internal::cut_tail(s0, line_x, to_remove, pieces);

        let a1 = s1.a;
        let b1 = Internal::cut_tail(s1, line_x, to_remove, pieces);

        let n = if a0 != a1 && b0 != b1 { 4 } else { 3 };

        let mut path = Vec::with_capacity(n);

        path.push(a0);
        if a0 != a1 {
            path.push(a1);
        }
        path.push(b1);
        if b0 != b1 {
            path.push(b0);
        }

        Some(path)
    }

    fn cut_tail(s: XSegment, x: i32, to_remove: &mut Vec<XSegment>, pieces: &mut Vec<XSegment>) -> IntPoint {
        if s.b.x > x {
            let tail = Internal::tail(&s, x);
            to_remove.push(s);
            let a = tail.a;
            pieces.push(tail);
            a
        } else {
            s.b
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithm::slice::slice::Slice;
    use crate::algorithm::slice::slice_type::SliceType;
    use crate::i_overlay::i_float::point::IntPoint;
    use crate::i_overlay::i_shape::int::path::PointPathExtension;

    #[test]
    fn test_0() {
        let shape = [
            [
                IntPoint::new(-1, -1),
                IntPoint::new(-1, 1),
                IntPoint::new(1, 1),
                IntPoint::new(1, -1)
            ].to_vec()
        ].to_vec();

        let result = shape.vertical_slice(SliceType::Original);
        assert_eq!(result.len(), 1);

        let a0 = result.iter().fold(0, |a, b| a + b.unsafe_area());
        let a1 = shape.iter().fold(0, |a, b| a + b.unsafe_area());
        assert_eq!(a0, a1);
    }

    #[test]
    fn test_1() {
        let shape = [
            [
                IntPoint::new(-1, 0),
                IntPoint::new(0, 1),
                IntPoint::new(1, 0),
                IntPoint::new(0, -1)
            ].to_vec()
        ].to_vec();

        let result = shape.vertical_slice(SliceType::Original);

        assert_eq!(result.len(), 2);

        let a0 = result.iter().fold(0, |a, b| a + b.unsafe_area());
        let a1 = shape.iter().fold(0, |a, b| a + b.unsafe_area());
        assert_eq!(a0, a1);
    }

    #[test]
    fn test_2() {
        let shape = [
            [
                IntPoint::new(-1, -1),
                IntPoint::new(-1, 1),
                IntPoint::new(1, 1),
                IntPoint::new(1, 0),
                IntPoint::new(0, 0),
                IntPoint::new(0, -1)
            ].to_vec()
        ].to_vec();

        let result = shape.vertical_slice(SliceType::Original);

        assert_eq!(result.len(), 2);

        let a0 = result.iter().fold(0, |a, b| a + b.unsafe_area());
        let a1 = shape.iter().fold(0, |a, b| a + b.unsafe_area());
        assert_eq!(a0, a1);
    }

    #[test]
    fn test_3() {
        let shape = [
            [
                IntPoint::new(-1, 0),
                IntPoint::new(-1, 1),
                IntPoint::new(1, 1),
                IntPoint::new(1, -1),
                IntPoint::new(0, -1),
                IntPoint::new(0, 0)
            ].to_vec()
        ].to_vec();

        let result = shape.vertical_slice(SliceType::Original);

        assert_eq!(result.len(), 2);

        let a0 = result.iter().fold(0, |a, b| a + b.unsafe_area());
        let a1 = shape.iter().fold(0, |a, b| a + b.unsafe_area());
        assert_eq!(a0, a1);
    }

    #[test]
    fn test_4() {
        let shape = [
            [
                IntPoint::new(-1, -1),
                IntPoint::new(-1, 0),
                IntPoint::new(0, 0),
                IntPoint::new(0, 1),
                IntPoint::new(1, 1),
                IntPoint::new(1, -1)
            ].to_vec()
        ].to_vec();

        let result = shape.vertical_slice(SliceType::Original);

        assert_eq!(result.len(), 2);

        let a0 = result.iter().fold(0, |a, b| a + b.unsafe_area());
        let a1 = shape.iter().fold(0, |a, b| a + b.unsafe_area());
        assert_eq!(a0, a1);
    }

    #[test]
    fn test_5() {
        let shape = [
            [
                IntPoint::new(-1, -1),
                IntPoint::new(-1, 1),
                IntPoint::new(0, 1),
                IntPoint::new(0, 0),
                IntPoint::new(1, 0),
                IntPoint::new(1, -1)
            ].to_vec()
        ].to_vec();

        let result = shape.vertical_slice(SliceType::Original);

        assert_eq!(result.len(), 2);

        let a0 = result.iter().fold(0, |a, b| a + b.unsafe_area());
        let a1 = shape.iter().fold(0, |a, b| a + b.unsafe_area());
        assert_eq!(a0, a1);
    }

    #[test]
    fn test_6() {
        let shape = [
            [
                IntPoint::new(-1, -1),
                IntPoint::new(-1, 1),
                IntPoint::new(0, 0),
            ].to_vec()
        ].to_vec();

        let result = shape.vertical_slice(SliceType::Original);

        assert_eq!(result.len(), 1);

        let a0 = result.iter().fold(0, |a, b| a + b.unsafe_area());
        let a1 = shape.iter().fold(0, |a, b| a + b.unsafe_area());
        assert_eq!(a0, a1);
    }

    #[test]
    fn test_7() {
        let shape = [
            [
                IntPoint::new(0, 0),
                IntPoint::new(1, 1),
                IntPoint::new(1, -1),
            ].to_vec()
        ].to_vec();

        let result = shape.vertical_slice(SliceType::Original);

        assert_eq!(result.len(), 1);

        let a0 = result.iter().fold(0, |a, b| a + b.unsafe_area());
        let a1 = shape.iter().fold(0, |a, b| a + b.unsafe_area());
        assert_eq!(a0, a1);
    }

    #[test]
    fn test_8() {
        let shape = [
            [
                IntPoint::new(-1, -2),
                IntPoint::new(-1, 2),
                IntPoint::new(0, 1),
                IntPoint::new(1, 2),
                IntPoint::new(1, -2),
                IntPoint::new(0, -1),
            ].to_vec()
        ].to_vec();

        let result = shape.vertical_slice(SliceType::Original);

        assert_eq!(result.len(), 2);

        let a0 = result.iter().fold(0, |a, b| a + b.unsafe_area());
        let a1 = shape.iter().fold(0, |a, b| a + b.unsafe_area());
        assert_eq!(a0, a1);
    }

    #[test]
    fn test_9() {
        let shape = [
            [
                IntPoint::new(-2, -1),
                IntPoint::new(-1, 0),
                IntPoint::new(-2, 1),
                IntPoint::new(2, 1),
                IntPoint::new(1, 0),
                IntPoint::new(2, -1),
            ].to_vec()
        ].to_vec();

        let result = shape.vertical_slice(SliceType::Original);

        assert_eq!(result.len(), 5);

        let a0 = result.iter().fold(0, |a, b| a + b.unsafe_area());
        let a1 = shape.iter().fold(0, |a, b| a + b.unsafe_area());
        assert_eq!(a0, a1);
    }

    #[test]
    fn test_10() {
        let shape = [
            [
                IntPoint::new(-1, 0),
                IntPoint::new(-2, 1),
                IntPoint::new(-1, 2),
                IntPoint::new(0, 1),
                IntPoint::new(1, 2),
                IntPoint::new(2, 1),
                IntPoint::new(1, 0),
                IntPoint::new(2, -1),
                IntPoint::new(1, -2),
                IntPoint::new(0, -1),
                IntPoint::new(-1, -2),
                IntPoint::new(-2, -1)
            ].to_vec()
        ].to_vec();

        let result = shape.vertical_slice(SliceType::Original);

        assert_eq!(result.len(), 6);

        let a0 = result.iter().fold(0, |a, b| a + b.unsafe_area());
        let a1 = shape.iter().fold(0, |a, b| a + b.unsafe_area());
        assert_eq!(a0, a1);
    }

    #[test]
    fn test_11() {
        let shape = [
            [
                IntPoint::new(-1, 0),
                IntPoint::new(0, 1),
                IntPoint::new(-2, 1),
                IntPoint::new(-2, 3),
                IntPoint::new(-1, 3),
                IntPoint::new(2, 0),
                IntPoint::new(-1, -3),
                IntPoint::new(-2, -3),
                IntPoint::new(-2, -1),
                IntPoint::new(0, -1)
            ].to_vec()
        ].to_vec();

        let result = shape.vertical_slice(SliceType::Original);

        assert_eq!(result.len(), 6);

        let a0 = result.iter().fold(0, |a, b| a + b.unsafe_area());
        let a1 = shape.iter().fold(0, |a, b| a + b.unsafe_area());
        assert_eq!(a0, a1);
    }

}