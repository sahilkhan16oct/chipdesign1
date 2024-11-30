use crate::export::shape::ExpShape;
use crate::i_overlay::i_float::point::IntPoint;
use crate::i_overlay::i_shape::int::path::IntPath;
use crate::i_overlay::i_shape::int::shape::IntShape;
use crate::task::task::VerificationResult;

pub trait PathToExpShape {
    fn into_exp_shapes(self) -> Vec<ExpShape>;
    fn into_task_result(self) -> VerificationResult;
}

pub trait EdgeToExpShape {
    fn into_exp_shapes(self) -> Vec<ExpShape>;
    fn into_task_result(self) -> VerificationResult;
}

impl PathToExpShape for Vec<IntShape> {
    fn into_exp_shapes(self) -> Vec<ExpShape> {
        let mut result = Vec::with_capacity(self.len());

        for shape in self.into_iter() {
            for path in shape.into_iter() {
                result.push(ExpShape::Path(path));
            }
        }
        result
    }

    fn into_task_result(self) -> VerificationResult {
        if self.is_empty() {
            VerificationResult::NoErrors
        } else {
            VerificationResult::Shape(self.into_exp_shapes())
        }
    }
}

impl PathToExpShape for Vec<IntPath> {
    fn into_exp_shapes(self) -> Vec<ExpShape> {
        let mut result = Vec::with_capacity(self.len());

        for path in self.into_iter() {
            result.push(ExpShape::Path(path));
        }
        result
    }

    fn into_task_result(self) -> VerificationResult {
        if self.is_empty() {
            VerificationResult::NoErrors
        } else {
            VerificationResult::Shape(self.into_exp_shapes())
        }
    }
}

impl PathToExpShape for Vec<IntPoint> {
    fn into_exp_shapes(self) -> Vec<ExpShape> {
        let mut result = Vec::with_capacity(self.len());

        for dot in self.into_iter() {
            result.push(ExpShape::Dot(dot));
        }
        result
    }

    fn into_task_result(self) -> VerificationResult {
        if self.is_empty() {
            VerificationResult::NoErrors
        } else {
            VerificationResult::Shape(self.into_exp_shapes())
        }
    }
}

impl EdgeToExpShape for Vec<[IntPoint; 3]> {
    fn into_exp_shapes(self) -> Vec<ExpShape> {
        let mut result = Vec::with_capacity(self.len());

        for path in self.iter() {
            let mut i = 1;
            while i < path.len() {
                let a = path[i - 1];
                let b = path[i];
                result.push(ExpShape::Edge([a, b]));
                i += 1
            }
        }
        result
    }

    fn into_task_result(self) -> VerificationResult {
        if self.is_empty() {
            VerificationResult::NoErrors
        } else {
            VerificationResult::Shape(self.into_exp_shapes())
        }
    }
}

impl EdgeToExpShape for Vec<[IntPoint; 2]> {
    fn into_exp_shapes(self) -> Vec<ExpShape> {
        let mut result = Vec::with_capacity(self.len());

        for path in self.iter() {
            let a = path[0];
            let b = path[1];
            if a == b {
                result.push(ExpShape::Dot(a));
            } else {
                result.push(ExpShape::Edge([a, b]));
            }
        }
        result
    }

    fn into_task_result(self) -> VerificationResult {
        if self.is_empty() {
            VerificationResult::NoErrors
        } else {
            VerificationResult::Shape(self.into_exp_shapes())
        }
    }
}