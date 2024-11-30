use foundation::i_overlay::i_float::point::IntPoint;
use foundation::task::direction::SpaceDirection;
use crate::solution::builder::SpaceBuilder;
use crate::solution::context::SpaceContext;

pub struct SpaceSolver {
    builder: SpaceBuilder,
    min_length: i32,
    direction: SpaceDirection,
}

impl SpaceSolver {
    pub fn new(builder: SpaceBuilder, min_length: i32, direction: SpaceDirection) -> Self {
        Self { builder, min_length, direction }
    }

    pub fn find_problems(&self) -> Vec<[IntPoint; 2]> {
        match &self.builder {
            SpaceBuilder::Single(builder) => {
                let space = builder.space();
                SpaceContext::new(space, self.min_length as i64, self.direction).find_problems()
            }
            SpaceBuilder::Double(builder) => {
                let space = builder.space();
                SpaceContext::new(space, self.min_length as i64, self.direction).find_problems()
            }
        }
    }
}