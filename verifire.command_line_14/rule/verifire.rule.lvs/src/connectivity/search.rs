use std::cmp::Ordering;
use std::collections::HashMap;
use foundation::fix::layer::LayerData;
use foundation::i_overlay::i_float::point::IntPoint;
use foundation::i_overlay::i_float::triangle::Triangle;
use foundation::i_overlay::i_shape::int::shape::{IntShape, IntShapes};
use foundation::i_overlay::id_point::IdPoint;
use foundation::i_overlay::x_segment::XSegment;
use foundation::i_tree::node::{Color, EMPTY_REF};
use foundation::i_tree::tree::Tree;
use crate::connectivity::device::Device;

pub(crate) struct Connection {
    pub(crate) point_id: u32,
    pub(crate) id: u32,
}

#[derive(Debug, Clone)]
struct Segment {
    id: u32,
    is_right: bool,
    x_segment: XSegment,
}

pub(crate) trait ShapeSearch {
    fn search(&self, points: &mut Vec<IdPoint>) -> Vec<Connection>;
    fn search_sorted(&self, points: &Vec<IdPoint>) -> Vec<Connection>;
}

pub(crate) trait DeviceSearch {
    fn search(&self, points: &mut Vec<IdPoint>, shape_store: &HashMap<LayerData, IntShapes>, layer_data: LayerData) -> Vec<Connection>;
    fn search_sorted(&self, points: &Vec<IdPoint>, shape_store: &HashMap<LayerData, IntShapes>, layer_data: LayerData) -> Vec<Connection>;
}

trait JoinSegments {
    fn join(&mut self, shape: &IntShape, id: u32, min_x: i32, max_x: i32);
}

impl ShapeSearch for IntShapes {
    fn search(&self, points: &mut Vec<IdPoint>) -> Vec<Connection> {
        points.sort_by(|p0, p1| p0.point.x.cmp(&p1.point.x));
        self.search_sorted(points)
    }
    fn search_sorted(&self, points: &Vec<IdPoint>) -> Vec<Connection> {
        let min_x = points[0].point.x;
        let max_x = points[points.len() - 1].point.x;

        let segments = Segment::segments(self, min_x, max_x);

        Solver::solve(&segments, &points)
    }
}

impl DeviceSearch for Vec<Device> {
    fn search(&self, points: &mut Vec<IdPoint>, shape_store: &HashMap<LayerData, IntShapes>, layer_data: LayerData) -> Vec<Connection> {
        points.sort_by(|p0, p1| p0.point.x.cmp(&p1.point.x));
        self.search_sorted(points, shape_store, layer_data)
    }

    fn search_sorted(&self, points: &Vec<IdPoint>, shape_store: &HashMap<LayerData, IntShapes>, layer_data: LayerData) -> Vec<Connection> {
        let min_x = points[0].point.x;
        let max_x = points[points.len() - 1].point.x;

        let shapes = if let Some(shapes) = shape_store.get(&layer_data) {
            shapes
        } else {
            return Vec::new();
        };

        let mut segments = Vec::with_capacity(4 * shapes.len().max(64));
        for device in self.iter() {
            let shape_indices = if let Some(shape_indices) = device.shapes.get(&layer_data) {
                shape_indices
            } else {
                continue;
            };

            for &shape_index in shape_indices.iter() {
                segments.join(&shapes[shape_index], device.index as u32, min_x, max_x);
            }
        }

        Solver::solve(&segments, &points)
    }
}

impl Segment {
    #[inline]
    pub fn is_under_point(&self, p: IntPoint) -> Ordering {
        let area = Triangle::area_two_point(self.x_segment.a, p, self.x_segment.b);
        if area == 0 {
            Ordering::Equal
        } else if area > 0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    fn segments(shapes: &IntShapes, min_x: i32, max_x: i32) -> Vec<Segment> {
        let mut segments = Vec::new();
        let mut id = 0;
        for shape in shapes.iter() {
            segments.join(shape, id, min_x, max_x);
            id += 1;
        }
        segments.sort_by(|s0, s1| s0.x_segment.a.x.cmp(&s1.x_segment.a.x));

        segments
    }
}

impl JoinSegments for Vec<Segment> {
    fn join(&mut self, shape: &IntShape, id: u32, min_x: i32, max_x: i32) {
        for path in shape.iter() {
            let mut a = path[path.len() - 1];
            for &b in path.iter() {
                if a.x != b.x {
                    let is_right = a.x < b.x;

                    let x_segment = if is_right {
                        XSegment::new(a, b)
                    } else {
                        XSegment::new(b, a)
                    };

                    if max_x >= x_segment.a.x && x_segment.b.x >= min_x {
                        self.push(Segment { id, is_right, x_segment })
                    }
                }
                a = b;
            }
        }
    }
}

struct Solver;

impl Solver {
    fn solve(segments: &[Segment], id_points: &[IdPoint]) -> Vec<Connection> {
        let mut result = Vec::with_capacity(id_points.len());

        let mut i = 0;
        let mut j = 0;
        let mut scan = ScanTree::new(id_points.len());

        while j < id_points.len() {
            let x = id_points[j].point.x;

            while i < segments.len() {
                let s = &segments[i];
                if s.x_segment.a.x > x {
                    break;
                }
                if s.x_segment.b.x >= x {
                    scan.insert(s.clone(), x)
                }
                i += 1;
            }

            while j < id_points.len() {
                let id_point = &id_points[j];
                if id_point.point.x != x {
                    break;
                }
                if let Some(s) = scan.find_under_and_nearest(id_point.point) {
                    if !s.is_right {
                        result.push(Connection { point_id: id_point.id as u32, id: s.id });
                    }
                }

                j += 1;
            }
        }

        result
    }
}


struct ScanTree {
    tree: Tree<Segment>,
}

impl ScanTree {
    #[inline]
    fn new(count: usize) -> Self {
        let capacity = (count as f64).sqrt() as usize;
        let x_segment = XSegment { a: IntPoint::ZERO, b: IntPoint::ZERO };
        Self { tree: Tree::new(Segment { id: 0, is_right: true, x_segment }, capacity) }
    }
}

impl ScanTree {
    fn insert(&mut self, segment: Segment, stop: i32) {
        let mut index = self.tree.root;
        let mut p_index = EMPTY_REF;
        let mut is_left = false;

        while index != EMPTY_REF {
            let node = self.tree.node(index);
            p_index = index;
            if node.value.x_segment.b == segment.x_segment.a {
                // replace
                self.tree.mut_node(index).value = segment;
                return;
            } else if node.value.x_segment.b.x < stop {
                let nd_parent = node.parent;
                _ = self.tree.delete_index(index);
                if nd_parent != EMPTY_REF {
                    index = nd_parent;
                } else {
                    index = self.tree.root;
                    p_index = EMPTY_REF;
                }
            } else {
                is_left = segment < node.value;
                if is_left {
                    index = node.left;
                } else {
                    index = node.right;
                }
            }
        }

        let new_index = self.tree.store.get_free_index();
        let new_node = self.tree.mut_node(new_index);
        new_node.left = EMPTY_REF;
        new_node.right = EMPTY_REF;
        new_node.color = Color::Red;
        new_node.value = segment;
        new_node.parent = p_index;

        if p_index == EMPTY_REF {
            self.tree.root = new_index;
        } else {
            if is_left {
                self.tree.mut_node(p_index).left = new_index;
            } else {
                self.tree.mut_node(p_index).right = new_index;
            }

            if self.tree.node(p_index).color == Color::Red {
                self.tree.fix_red_black_properties_after_insert(new_index, p_index)
            }
        }
    }

    fn find_under_and_nearest(&mut self, p: IntPoint) -> Option<Segment> {
        let mut index = self.tree.root;
        let mut result = EMPTY_REF;
        while index != EMPTY_REF {
            let node = self.tree.node(index);
            if node.value.x_segment.b.x < p.x {
                let nd_parent = node.parent;
                _ = self.tree.delete_index(index);
                if nd_parent != EMPTY_REF {
                    index = nd_parent;
                } else {
                    index = self.tree.root;
                }
            } else {
                match node.value.is_under_point(p) {
                    Ordering::Less => {
                        result = index;
                        index = node.right;
                    }
                    Ordering::Equal => {
                        let mut s = node.value.clone();
                        s.is_right = false;
                        return Some(s);
                    }
                    Ordering::Greater => {
                        index = node.left;
                    }
                }
            }
        }

        if result == EMPTY_REF {
            None
        } else {
            Some(self.tree.node(result).value.clone())
        }
    }
}

impl PartialEq<Self> for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.x_segment.eq(&other.x_segment)
    }
}

impl Eq for Segment {}

impl PartialOrd for Segment {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Segment {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x_segment.is_under_segment(&other.x_segment) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}