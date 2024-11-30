use serde::{Deserialize, Serialize};
use foundation::i_overlay::i_float::point::IntPoint;
use foundation::i_overlay::i_shape::int::shape::IntShapes;
use foundation::res::shape_res::ShapeRes;
use foundation::support::fix::ToFixShape;
use foundation::task::orientation::Orientation;
use crate::solution::min_diagonal::MinDiagonal;
use crate::solution::min_max_width::MinMaxWidth;

#[derive(Debug, Clone, Copy)]
pub enum Condition {
    Max(i32),
    Min(i32),
}

enum MinMaxAction {
    XY(Condition, Orientation),
    Diagonal(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MinMaxAlgorithm {
    #[serde(rename = "min_width_x")]
    MinWidthX,
    #[serde(rename = "min_width_y")]
    MinWidthY,
    #[serde(rename = "min_width_xy")]
    MinWidthXY,
    #[serde(rename = "min_width_dxy")]
    MinWidthDXY,
    #[serde(rename = "max_width_x")]
    MaxWidthX,
    #[serde(rename = "max_width_y")]
    MaxWidthY,
    #[serde(rename = "max_width_xy")]
    MaxWidthXY,
}

pub struct MinMaxSolver {
    shape_res: ShapeRes,
    algorithm: MinMaxAlgorithm,
    value: i32,
}

impl MinMaxSolver {
    pub fn new(shape_res: ShapeRes, algorithm: MinMaxAlgorithm, value: i32) -> Self {
        Self { shape_res, algorithm, value }
    }

    pub fn find_problems(&self) -> Vec<[IntPoint; 2]> {
        let shapes = self.shape_res.shapes();
        Self::solve(shapes, self.algorithm, self.value)
    }

    pub fn solve(shapes: IntShapes, algorithm: MinMaxAlgorithm, value: i32) -> Vec<[IntPoint; 2]> {
        let actions = Self::actions(algorithm, value);
        let mut result = Vec::new();
        for shape in shapes.iter() {
            for action in actions.iter() {
                match action {
                    MinMaxAction::XY(condition, direction) => {
                        let edges = shape.find_xy_problems(*direction, *condition);
                        if !edges.is_empty() {
                            for e in edges.iter() {
                                result.push([e.a, e.b])
                            }
                        }
                    }
                    MinMaxAction::Diagonal(min_length) => {
                        let fix_shape = shape.to_fix();
                        let edges = fix_shape.find_diagonal_problems(*min_length);
                        if !edges.is_empty() {
                            for e in edges.iter() {
                                result.push([e.a, e.b])
                            }
                        }
                    }
                }
            }
        }

        result
    }

    fn actions(algorithm: MinMaxAlgorithm, value: i32) -> Vec<MinMaxAction> {
        let mut actions = Vec::with_capacity(2);
        match algorithm {
            MinMaxAlgorithm::MinWidthX => {
                actions.push(MinMaxAction::XY(Condition::Min(value), Orientation::Horizontal));
            }
            MinMaxAlgorithm::MinWidthY => {
                actions.push(MinMaxAction::XY(Condition::Min(value), Orientation::Vertical));
            }
            MinMaxAlgorithm::MinWidthXY => {
                actions.push(MinMaxAction::XY(Condition::Min(value), Orientation::Horizontal));
                actions.push(MinMaxAction::XY(Condition::Min(value), Orientation::Vertical));
            }
            MinMaxAlgorithm::MinWidthDXY => {
                actions.push(MinMaxAction::Diagonal(value));
            }
            MinMaxAlgorithm::MaxWidthX => {
                actions.push(MinMaxAction::XY(Condition::Max(value), Orientation::Horizontal));
            }
            MinMaxAlgorithm::MaxWidthY => {
                actions.push(MinMaxAction::XY(Condition::Max(value), Orientation::Vertical));
            }
            MinMaxAlgorithm::MaxWidthXY => {
                actions.push(MinMaxAction::XY(Condition::Max(value), Orientation::Horizontal));
                actions.push(MinMaxAction::XY(Condition::Max(value), Orientation::Vertical));
            }
        }
        actions
    }
}