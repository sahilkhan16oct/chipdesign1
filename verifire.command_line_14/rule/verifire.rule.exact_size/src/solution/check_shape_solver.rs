use foundation::res::shape_res::ShapeRes;
use serde::{Deserialize, Serialize};
use foundation::i_overlay::i_shape::int::path::IntPath;
use crate::solution::rect::RectShape;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ShapeForm {
    #[serde(rename = "square")]
    Square,
    #[serde(rename = "vertical")]
    Vertical,
    #[serde(rename = "horizontal")]
    Horizontal,
}

pub struct CheckShapeSolver {
    shape_res: ShapeRes,
    form: ShapeForm,
}

impl CheckShapeSolver {
    pub fn new(shape_res: ShapeRes, form: ShapeForm) -> Self {
        Self {
            shape_res,
            form,
        }
    }

    pub fn find_problems(&self) -> Vec<IntPath> {
        let shapes = self.shape_res.shapes();

        let mut result = Vec::new();

        for shape in shapes.iter() {
            if let Some(size) = shape.size() {
                match self.form {
                    ShapeForm::Square => {
                        if size.width == size.height {
                            continue
                        }
                    }
                    ShapeForm::Vertical => {
                        if size.width < size.height {
                            continue
                        }
                    }
                    ShapeForm::Horizontal => {
                        if size.width > size.height {
                            continue
                        }
                    }
                }
            }
            result.push(shape[0].clone());
        }

        result
    }
}


#[cfg(test)]
mod tests {
    use foundation::i_overlay::i_float::point::IntPoint;
    use foundation::res::shape_res::ShapeRes;
    use crate::solution::check_shape_solver::{CheckShapeSolver, ShapeForm};

    #[test]
    fn test_0() {
        let shapes = [
            [
                [
                    IntPoint::new(-10, -10),
                    IntPoint::new(-10, 10),
                    IntPoint::new(10, 10),
                    IntPoint::new(10, -10)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        assert_eq!(CheckShapeSolver::new(ShapeRes::new(shapes.clone()), ShapeForm::Square).find_problems().is_empty(), true);
        assert_eq!(CheckShapeSolver::new(ShapeRes::new(shapes.clone()), ShapeForm::Vertical).find_problems().is_empty(), false);
        assert_eq!(CheckShapeSolver::new(ShapeRes::new(shapes.clone()), ShapeForm::Horizontal).find_problems().is_empty(), false);
    }

    #[test]
    fn test_1() {
        let shapes = [
            [
                [
                    IntPoint::new(-100, -10),
                    IntPoint::new(-100, 10),
                    IntPoint::new(100, 10),
                    IntPoint::new(100, -10)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        assert_eq!(CheckShapeSolver::new(ShapeRes::new(shapes.clone()), ShapeForm::Square).find_problems().is_empty(), false);
        assert_eq!(CheckShapeSolver::new(ShapeRes::new(shapes.clone()), ShapeForm::Vertical).find_problems().is_empty(), false);
        assert_eq!(CheckShapeSolver::new(ShapeRes::new(shapes.clone()), ShapeForm::Horizontal).find_problems().is_empty(), true);
    }

    #[test]
    fn test_2() {
        let shapes = [
            [
                [
                    IntPoint::new(-10, -100),
                    IntPoint::new(-10, 100),
                    IntPoint::new(10, 100),
                    IntPoint::new(10, -100)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        assert_eq!(CheckShapeSolver::new(ShapeRes::new(shapes.clone()), ShapeForm::Square).find_problems().is_empty(), false);
        assert_eq!(CheckShapeSolver::new(ShapeRes::new(shapes.clone()), ShapeForm::Vertical).find_problems().is_empty(), true);
        assert_eq!(CheckShapeSolver::new(ShapeRes::new(shapes.clone()), ShapeForm::Horizontal).find_problems().is_empty(), false);
    }

    #[test]
    fn test_3() {
        let shapes = [
            [
                [
                    IntPoint::new(-10, 0),
                    IntPoint::new(0, 10),
                    IntPoint::new(10, 0),
                    IntPoint::new(0, -10)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        assert_eq!(CheckShapeSolver::new(ShapeRes::new(shapes.clone()), ShapeForm::Square).find_problems().is_empty(), false);
        assert_eq!(CheckShapeSolver::new(ShapeRes::new(shapes.clone()), ShapeForm::Vertical).find_problems().is_empty(), false);
        assert_eq!(CheckShapeSolver::new(ShapeRes::new(shapes.clone()), ShapeForm::Horizontal).find_problems().is_empty(), false);
    }
}

