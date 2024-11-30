use foundation::i_overlay::i_shape::int::shape::IntShapes;
use foundation::res::shape_res::ShapeRes;

pub struct FilterSolver {
    pub (super) shapes_res: ShapeRes
}

impl FilterSolver {
    pub fn new(shapes_res: ShapeRes) -> Self {
        Self { shapes_res }
    }

    pub fn find_problems(&self) -> IntShapes {
        self.shapes_res.shapes()
    }
}