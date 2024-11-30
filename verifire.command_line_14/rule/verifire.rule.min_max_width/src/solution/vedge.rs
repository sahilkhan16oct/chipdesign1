#[derive(Debug, Clone, Copy)]
pub(super) struct VEdge {
    pub (super) y0: i32,
    pub (super) y1: i32
}

impl VEdge {

    pub (super) fn is_zero(&self) -> bool {
        self.y0 == self.y1
    }
    pub (super) fn new(y0: i32, y1: i32) -> Self {
        Self { y0, y1}
    }

    pub (super) fn with_y(y: i32) -> Self {
        Self { y0: y, y1: y}
    }

    // applicable only in ascending order
    pub (super) fn join(&mut self, edge: Self) -> bool {
        return if self.y1 == edge.y0 {
            self.y1 = edge.y1;
            true
        } else {
            false
        }
    }

    pub (super) fn union(&mut self, edge: Self) -> bool {
        if self.y1 < edge.y0 || self.y0 > edge.y1 {
            return false
        }

        self.y0 = self.y0.min(edge.y0);
        self.y1 = self.y1.max(edge.y1);

        return true
    }
}

pub(super) trait Unionable {
    fn union(&mut self);
}

impl Unionable for Vec<VEdge> {
    fn union(&mut self) {
        let mut i = 0;
        while i < self.len() - 1 {
            let mut ei = self[i];
            let j0 = i + 1;
            let mut j = j0;
            while j < self.len() {
                let ej = self[j];
                if ei.union(ej) {
                    self.swap_remove(j);
                    j = j0;
                } else {
                    j += 1;
                }
            }
            self[i] = ei;
            i += 1;
        }
    }
}