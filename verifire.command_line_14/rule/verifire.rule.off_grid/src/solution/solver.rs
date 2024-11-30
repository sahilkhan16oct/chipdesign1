use foundation::i_overlay::i_float::point::IntPoint;
use foundation::res::shape_res::ShapeRes;

pub struct OffGridSolver {
    shape_res: ShapeRes,
    step: i32,
}

impl OffGridSolver {
    pub fn new(shape_res: ShapeRes, step: i32) -> Self {
        Self { shape_res, step }
    }

    pub fn find_problems(&self) -> Vec<IntPoint> {
        let shapes = self.shape_res.shapes();
        let mut dots = Vec::new();

        for shape in shapes.iter() {
            for path in shape.iter() {
                for p in path.iter() {
                    if p.x % self.step != 0 || p.y % self.step != 0 {
                        dots.push(p.clone());
                    }
                }
            }
        }

        dots
    }
}