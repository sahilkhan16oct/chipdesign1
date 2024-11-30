use foundation::algorithm::slice::slice::Slice;
use foundation::algorithm::slice::slice_type::SliceType;
use foundation::i_overlay::i_shape::int::path::IntPath;
use foundation::i_overlay::i_shape::int::shape::IntShapes;
use foundation::task::orientation::Orientation;

pub struct SliceSolver;

impl SliceSolver {
    pub fn slice(shapes: IntShapes, orientation: Orientation, slice_type: SliceType) -> Vec<IntPath> {
        let mut shapes = shapes;
        let mut result = Vec::new();
        for shape in shapes.iter_mut() {
            match orientation {
                Orientation::Vertical => {
                    let mut rects = shape.vertical_slice(slice_type);
                    result.append(&mut rects)
                }
                Orientation::Horizontal => {
                    shape.rotate_clock_wise();
                    let mut rects = shape.vertical_slice(slice_type);
                    rects.rotate_counter_clock_wise();
                    result.append(&mut rects)
                }
            }
        }

        result
    }
}

trait Rotate {
    fn rotate_clock_wise(&mut self);
    fn rotate_counter_clock_wise(&mut self);
}

impl Rotate for Vec<IntPath> {
    fn rotate_clock_wise(&mut self) {
        for path in self.iter_mut() {
            for p in path.iter_mut() {
                let x = p.x;
                p.x = p.y;
                p.y = -x;
            }
        }
    }
    fn rotate_counter_clock_wise(&mut self) {
        for path in self.iter_mut() {
            for p in path.iter_mut() {
                let x = p.x;
                p.x = -p.y;
                p.y = x;
            }
        }
    }
}