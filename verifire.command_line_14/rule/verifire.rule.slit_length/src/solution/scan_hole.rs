use foundation::i_overlay::bind::segment::IdSegment;
use foundation::i_overlay::i_float::point::IntPoint;
use foundation::i_overlay::x_segment::XSegment;
use foundation::i_tree::node::{Color, EMPTY_REF};
use foundation::i_tree::tree::Tree;

pub(super) struct ScanHoleTree {
    tree: Tree<IdSegment>,
}

impl ScanHoleTree {
    pub(super) fn new(count: usize) -> Self {
        let x_segment = XSegment { a: IntPoint::ZERO, b: IntPoint::ZERO };
        let segment = IdSegment { id: usize::MAX, x_segment };
        Self { tree: Tree::new(segment, (count as f64).sqrt() as usize) }
    }

    pub(super) fn insert(&mut self, segment: IdSegment, stop: i32) {
        let mut index = self.tree.root;
        let mut p_index = EMPTY_REF;
        let mut is_left = false;

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

    pub(super) fn find_under_and_nearest(&mut self, p: IntPoint, stop: i32) -> usize {
        let mut index = self.tree.root;
        let mut result = EMPTY_REF;
        while index != EMPTY_REF {
            let node = self.tree.node(index);
            if node.value.x_segment.b.x <= stop {
                let nd_parent = node.parent;
                _ = self.tree.delete_index(index);
                if nd_parent != EMPTY_REF {
                    index = nd_parent;
                } else {
                    index = self.tree.root;
                }
            } else {
                if node.value.x_segment.is_under_point(p) {
                    result = index;
                    index = node.right;
                } else {
                    index = node.left;
                }
            }
        }

        if result != EMPTY_REF {
            self.tree.node(result).value.id
        } else {
            usize::MAX
        }
    }

    pub(super) fn find_above_and_nearest(&mut self, p: IntPoint, stop: i32) -> usize {
        let mut index = self.tree.root;
        let mut result = EMPTY_REF;
        while index != EMPTY_REF {
            let node = self.tree.node(index);
            if node.value.x_segment.b.x <= stop {
                let nd_parent = node.parent;
                _ = self.tree.delete_index(index);
                if nd_parent != EMPTY_REF {
                    index = nd_parent;
                } else {
                    index = self.tree.root;
                }
            } else {
                if node.value.x_segment.is_under_point(p) {
                    index = node.right;
                } else {
                    result = index;
                    index = node.left;
                }
            }
        }

        if result != EMPTY_REF {
            self.tree.node(result).value.id
        } else {
            usize::MAX
        }
    }
}