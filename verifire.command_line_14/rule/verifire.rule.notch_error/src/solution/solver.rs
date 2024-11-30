use foundation::i_overlay::i_float::point::IntPoint;
use foundation::i_overlay::i_shape::int::path::IntPath;
use foundation::res::shape_res::ShapeRes;

pub struct NotchSolver {
    shape_res: ShapeRes,
    sqr_length: i64,
}

impl NotchSolver {
    pub fn new(shape_res: ShapeRes, min_length: i32) -> Self {
        let l = min_length as i64;
        Self { shape_res, sqr_length: l * l }
    }

    pub fn find_problems(&self) -> Vec<[IntPoint; 2]> {
        let shapes = self.shape_res.shapes();
        let mut result = Vec::new();
        for shape in shapes.iter() {
            for path in shape.iter() {
                Self::verify_notch(path, self.sqr_length, &mut result)
            }
        }

        result
    }

    fn verify_notch(path: &IntPath, sqr_length: i64, result: &mut Vec<[IntPoint; 2]>) {
        let n = path.len();
        let mut a = path[n - 2];
        let mut b = path[n - 1];

        let mut ab = a.sqr_distance(b);
        let mut is_ab_added = false;

        for &c in path.iter() {
            let bc = b.sqr_distance(c);

            let is_problem = ab < sqr_length && bc < sqr_length;
            if is_problem {
                if !is_ab_added {
                    result.push([a, b]);
                }

                result.push([b, c]);
            }

            is_ab_added = is_problem;

            ab = bc;
            a = b;
            b = c;
        }
    }
}


#[cfg(test)]
mod tests {
    use foundation::i_overlay::i_float::point::IntPoint;
    use foundation::res::shape_res::ShapeRes;
    use crate::solution::solver::NotchSolver;

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

        let result = NotchSolver::new(ShapeRes::new(shapes), 20).find_problems();

        assert_eq!(result.is_empty(), true);
    }

    #[test]
    fn test_1() {
        let shapes = [
            [
                [
                    IntPoint::new(-10, -10),
                    IntPoint::new(-10, 10),
                    IntPoint::new(5, 10),
                    IntPoint::new(5, 5),
                    IntPoint::new(10, 5),
                    IntPoint::new(10, -10)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let result = NotchSolver::new(ShapeRes::new(shapes), 6).find_problems();

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_2() {
        let shapes = [
            [
                [
                    IntPoint::new(-10, -10),
                    IntPoint::new(-10, 10),
                    IntPoint::new(-5, 10),
                    IntPoint::new(-5, 5),
                    IntPoint::new(5, 5),
                    IntPoint::new(5, 10),
                    IntPoint::new(10, 10),
                    IntPoint::new(10, -10)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let result = NotchSolver::new(ShapeRes::new(shapes), 6).find_problems();

        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_3() {
        let shapes = [
            [
                [
                    IntPoint::new(-10, -10),
                    IntPoint::new(-10, 10),
                    IntPoint::new(-5, 10),
                    IntPoint::new(-5, 5),
                    IntPoint::new(0, 5),
                    IntPoint::new(0, 10),
                    IntPoint::new(5, 10),
                    IntPoint::new(5, 5),
                    IntPoint::new(10, 5),
                    IntPoint::new(10, 0),
                    IntPoint::new(5, 0),
                    IntPoint::new(5, -5),
                    IntPoint::new(10, -5),
                    IntPoint::new(10, -10)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let result = NotchSolver::new(ShapeRes::new(shapes), 6).find_problems();

        assert_eq!(result.len(), 12);
    }
}