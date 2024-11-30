use std::cmp::Ordering;
use crate::i_overlay::i_float::point::IntPoint;
use crate::i_overlay::i_float::triangle::Triangle;
use crate::i_overlay::x_segment::XSegment;
use crate::i_tree::node::{Color, EMPTY_REF};
use crate::i_tree::tree::Tree;

#[derive(Debug, Clone, Copy)]
pub(super) struct ScanSegment {
    pub(super) x_segment: XSegment,
    pub(super) is_visited: bool,
}

impl PartialEq<Self> for ScanSegment {
    fn eq(&self, other: &Self) -> bool {
        self.x_segment == other.x_segment
    }
}

impl Eq for ScanSegment {}

impl PartialOrd for ScanSegment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScanSegment {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x_segment.is_under_segment(&other.x_segment) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl ScanSegment {
    fn is_under_point(&self, p: IntPoint) -> bool {
        debug_assert!(self.x_segment.a.x <= p.x && p.x <= self.x_segment.b.x);
        Triangle::is_clockwise_point(self.x_segment.a, p, self.x_segment.b)
    }

    fn is_segment(&self, s: XSegment) -> Ordering {
        let is_a = Triangle::is_line_point(s.a, self.x_segment.a, self.x_segment.b);
        let is_b = Triangle::is_line_point(s.b, self.x_segment.a, self.x_segment.b);
        if is_a && is_b {
            Ordering::Equal
        } else if self.x_segment.is_under_segment(&s) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }

    fn is_above_point(&self, p: IntPoint) -> bool {
        debug_assert!(self.x_segment.a.x <= p.x && p.x <= self.x_segment.b.x);
        Triangle::is_clockwise_point(p, self.x_segment.a, self.x_segment.b)
    }
}

pub(super) struct ScanTree {
    pub(super) tree: Tree<ScanSegment>,
}

impl ScanTree {
    pub(super) fn new(count: usize) -> Self {
        let capacity = count.log2_sqrt();
        Self { tree: Tree::new(ScanSegment { x_segment: XSegment { a: IntPoint::ZERO, b: IntPoint::ZERO }, is_visited: false }, capacity) }
    }

    pub(super) fn insert(&mut self, x_segment: XSegment) {
        let mut index = self.tree.root;
        let mut p_index = EMPTY_REF;
        let mut is_left = false;

        let stop = x_segment.a.x;
        let segment = ScanSegment { x_segment, is_visited: false };

        while index != EMPTY_REF {
            let node = self.tree.node(index);
            p_index = index;
            if node.value.x_segment.b.x <= stop {
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

    pub(super) fn find_under_point(&mut self, p: IntPoint) -> u32 {
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
                if node.value.is_under_point(p) {
                    result = index;
                    index = node.right;
                } else {
                    index = node.left;
                }
            }
        }

        self.validate(result)
    }

    pub(super) fn find_under_segment(&mut self, s: XSegment) -> u32 {
        let mut index = self.tree.root;
        let mut result = index;
        while index != EMPTY_REF {
            let node = self.tree.node(index);
            if node.value.x_segment.b.x < s.a.x {
                let nd_parent = node.parent;
                _ = self.tree.delete_index(index);
                if nd_parent != EMPTY_REF {
                    index = nd_parent;
                } else {
                    index = self.tree.root;
                }
            } else {
                if node.value.x_segment.is_under_segment(&s) {
                    result = index;
                    index = node.right;
                } else {
                    index = node.left;
                }
            }
        }

        self.validate(result)
    }

    pub(super) fn find_above_point(&mut self, p: IntPoint) -> u32 {
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
                if node.value.is_above_point(p) {
                    result = index;
                    index = node.left;
                } else {
                    index = node.right;
                }
            }
        }

        self.validate(result)
    }

    pub(super) fn find_above_segment(&mut self, s: XSegment) -> u32 {
        let mut index = self.tree.root;
        let mut result = index;
        while index != EMPTY_REF {
            let node = self.tree.node(index);
            if node.value.x_segment.b.x < s.a.x {
                let nd_parent = node.parent;
                _ = self.tree.delete_index(index);
                if nd_parent != EMPTY_REF {
                    index = nd_parent;
                } else {
                    index = self.tree.root;
                }
            } else {
                match node.value.is_segment(s) {
                    Ordering::Less => {
                        result = index;
                        index = node.left;
                    }
                    Ordering::Equal => {
                        index = node.right;
                    }
                    Ordering::Greater => {
                        index = node.right;
                    }
                }
            }
        }

        self.validate(result)
    }

    pub(super) fn find_segment(&mut self, s: XSegment) -> u32 {
        let mut index = self.tree.root;
        while index != EMPTY_REF {
            let node = self.tree.node(index);
            if node.value.x_segment.b.x < s.b.x {
                let nd_parent = node.parent;
                _ = self.tree.delete_index(index);
                if nd_parent != EMPTY_REF {
                    index = nd_parent;
                } else {
                    index = self.tree.root;
                }
            } else {
                match node.value.is_segment(s) {
                    Ordering::Less => {
                        index = node.left;
                    }
                    Ordering::Equal => {
                        return self.validate(index);
                    }
                    Ordering::Greater => {
                        index = node.right;
                    }
                }
            }
        }

        EMPTY_REF
    }

    pub(super) fn segment(&self, index: u32) -> XSegment {
        self.tree.node(index).value.x_segment
    }

    pub(super) fn make_visited(&mut self, index: u32) {
        self.tree.mut_node(index).value.is_visited = true
    }

    fn validate(&self, result: u32) -> u32 {
        if result != EMPTY_REF {
            let segment = self.tree.node(result);
            if !segment.value.is_visited {
                result
            } else {
                EMPTY_REF
            }
        } else {
            EMPTY_REF
        }
    }
}

trait Int {
    fn log2_sqrt(&self) -> usize;
}

impl Int for usize {
    fn log2_sqrt(&self) -> usize {
        let z = self.leading_zeros();
        let i = (usize::BITS - z) as usize;
        let n = (i + 1) >> 1;
        1 << n
    }
}