use foundation::i_overlay::i_shape::int::shape::IntShapes;
use foundation::res::shape_res::ShapeRes;
use crate::solution::ab_space::ABSpace;
use crate::solution::single_space::SingleSpace;

pub enum SpaceBuilder {
    Single(SingleSpaceBuilder),
    Double(DoubleSpaceBuilder),
}

impl SpaceBuilder {
    pub fn new(main: ShapeRes, second: Option<ShapeRes>) -> Self {
        if let Some(second) = second {
            SpaceBuilder::Double(DoubleSpaceBuilder { res_a: main, res_b: second })
        } else {
            SpaceBuilder::Single(SingleSpaceBuilder { res: main })
        }
    }
    pub fn new_shapes(shapes: IntShapes) -> Self {
        SpaceBuilder::Single(SingleSpaceBuilder { res: ShapeRes::new(shapes) })
    }
}

pub struct SingleSpaceBuilder {
    res: ShapeRes,
}

pub struct DoubleSpaceBuilder {
    res_a: ShapeRes,
    res_b: ShapeRes,
}

impl SingleSpaceBuilder {
    pub(super) fn space(&self) -> SingleSpace {
        SingleSpace { shapes: self.res.shapes() }
    }
}

impl DoubleSpaceBuilder {
    pub(super) fn space(&self) -> ABSpace {
        ABSpace { shapes_a: self.res_a.shapes(), shapes_b: self.res_b.shapes() }
    }
}