use foundation::i_overlay::i_shape::int::path::IntPath;
use foundation::res::shape_res::ShapeRes;
use crate::solution::rect::RectShape;

pub struct ExactSizeSolver {
    shape_res: ShapeRes,
    width: i32,
    height: i32,
}

impl ExactSizeSolver {
    pub fn new(shape_res: ShapeRes, width: i32, height: i32) -> Self {
        Self {
            shape_res,
            width,
            height,
        }
    }

    pub fn find_problems(&self) -> Vec<IntPath> {
        let shapes = self.shape_res.shapes();

        let mut result = Vec::new();

        for shape in shapes.iter() {
            if let Some(size) = shape.size() {
                let is_same = self.width == size.width && self.height == size.height
                    || self.width == size.height && self.height == size.width;
                if is_same {
                    continue;
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
    use crate::solution::exact_size_solver::ExactSizeSolver;

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

        let a: i32 = 20;
        let b: i32 = 20;

        let result = ExactSizeSolver::new(ShapeRes::new(shapes), a, b).find_problems();

        assert_eq!(result.is_empty(), true);
    }

    #[test]
    fn test_1() {
        let shapes = [
            [
                [
                    IntPoint::new(-20, -10),
                    IntPoint::new(-20, 10),
                    IntPoint::new(20, 10),
                    IntPoint::new(20, -10)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let a: i32 = 40;
        let b: i32 = 20;

        let result = ExactSizeSolver::new(ShapeRes::new(shapes), a, b).find_problems();

        assert_eq!(result.is_empty(), true);
    }

    #[test]
    fn test_2() {
        let shapes = [
            [
                [
                    IntPoint::new(-10, -20),
                    IntPoint::new(-10, 20),
                    IntPoint::new(10, 20),
                    IntPoint::new(10, -20)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let a: i32 = 20;
        let b: i32 = 40;

        let result = ExactSizeSolver::new(ShapeRes::new(shapes), a, b).find_problems();

        assert_eq!(result.is_empty(), true);
    }

    #[test]
    fn test_3() {
        let shapes = [
            [
                [
                    IntPoint::new(-10, -10),
                    IntPoint::new(-20, 0),
                    IntPoint::new(-10, 10),
                    IntPoint::new(10, 10),
                    IntPoint::new(10, -10)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let a: i32 = 20;
        let b: i32 = 20;

        let result = ExactSizeSolver::new(ShapeRes::new(shapes), a, b).find_problems();

        assert_eq!(result.is_empty(), false);
    }

    #[test]
    fn test_4() {
        let shapes = [
            [
                [
                    IntPoint::new(-11, -10),
                    IntPoint::new(-10, 10),
                    IntPoint::new(10, 10),
                    IntPoint::new(10, -10)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let a: i32 = 20;
        let b: i32 = 20;

        let result = ExactSizeSolver::new(ShapeRes::new(shapes), a, b).find_problems();

        assert_eq!(result.is_empty(), false);
    }
}