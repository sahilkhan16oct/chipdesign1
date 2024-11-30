use foundation::i_overlay::i_float::fix_vec::FixVec;
use foundation::i_overlay::i_float::point::IntPoint;
use foundation::i_overlay::i_shape::int::path::IntPath;
use foundation::res::shape_res::ShapeRes;

pub struct AngleSolver {
    pub(super) shapes_res: ShapeRes,
    pub(super) is_45_allowed: bool,
}

impl AngleSolver {
    pub fn new(shapes_res: ShapeRes, is_45_allowed: bool) -> Self {
        Self { shapes_res, is_45_allowed }
    }

    pub fn find_problems(&self) -> Vec<[IntPoint; 3]> {
        let shapes = self.shapes_res.shapes();
        let mut result = Vec::new();

        for shape in shapes.iter() {
            for path in shape.iter() {
                verify_angles(path, self.is_45_allowed, &mut result);
            }
        }

        result
    }
}

fn is_valid(ab: &FixVec, bc: FixVec, is_45_allowed: bool) -> bool {
    let dot = ab.dot_product(bc);

    if dot == 0 {
        return true;
    }

    if dot < 0 {
        return false;
    }

    let cross = ab.cross_product(bc);
    if cross == 0 {
        return true;
    }

    is_45_allowed && dot == cross.abs()
}

fn verify_angles(path: &IntPath, is_45_allowed: bool, result: &mut Vec<[IntPoint; 3]>) {
    let n = path.len();
    let mut a = path[n - 2];
    let mut b = path[n - 1];

    let mut ab = b.subtract(a);

    for &c in path.iter() {
        let bc = c.subtract(b);

        if !is_valid(&ab, bc, is_45_allowed) {
            let path = [a, b, c];
            result.push(path);
        }

        ab = bc;
        a = b;
        b = c;
    }
}

#[cfg(test)]
mod tests {
    use foundation::i_overlay::i_float::point::IntPoint;
    use foundation::res::shape_res::ShapeRes;
    use crate::solution::solver::AngleSolver;

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

        let result = AngleSolver::new(ShapeRes::new(shapes), true).find_problems();

        assert_eq!(result.is_empty(), true);
    }

    #[test]
    fn test_1() {
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

        let result_0 = AngleSolver::new(ShapeRes::new(shapes.clone()), true).find_problems();
        let result_1 = AngleSolver::new(ShapeRes::new(shapes), false).find_problems();

        assert_eq!(result_0.is_empty(), false);
        assert_eq!(result_1.is_empty(), false);
    }

    #[test]
    fn test_2() {
        let shapes = [
            [
                [
                    IntPoint::new(-15, -5),
                    IntPoint::new(-15, 5),
                    IntPoint::new(-5, 15),
                    IntPoint::new(5, 15),
                    IntPoint::new(15, 5),
                    IntPoint::new(15, -5),
                    IntPoint::new(5, -15),
                    IntPoint::new(-5, -15)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let result_0 = AngleSolver::new(ShapeRes::new(shapes.clone()), true).find_problems();
        let result_1 = AngleSolver::new(ShapeRes::new(shapes), false).find_problems();

        assert_eq!(result_0.is_empty(), true);
        assert_eq!(result_1.is_empty(), false);
    }

    #[test]
    fn test_3() {
        let shapes = [
            [
                [
                    IntPoint::new(-5, -15),
                    IntPoint::new(5, -15),
                    IntPoint::new(15, -5),
                    IntPoint::new(15, 5),
                    IntPoint::new(5, 15),
                    IntPoint::new(-5, 15),
                    IntPoint::new(-15, 5),
                    IntPoint::new(-15, -5)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let result_0 = AngleSolver::new(ShapeRes::new(shapes.clone()), true).find_problems();
        let result_1 = AngleSolver::new(ShapeRes::new(shapes), false).find_problems();

        assert_eq!(result_0.is_empty(), true);
        assert_eq!(result_1.is_empty(), false);
    }

    #[test]
    fn test_4() {
        let shapes = [
            [
                [
                    IntPoint::new(-15, -5),
                    IntPoint::new(-50, -5),
                    IntPoint::new(-50, 5),
                    IntPoint::new(-15, 5),
                    IntPoint::new(-5, 15),
                    IntPoint::new(5, 15),
                    IntPoint::new(15, 5),
                    IntPoint::new(50, 5),
                    IntPoint::new(50, -5),
                    IntPoint::new(15, -5),
                    IntPoint::new(5, -15),
                    IntPoint::new(-5, -15)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let result_0 = AngleSolver::new(ShapeRes::new(shapes.clone()), true).find_problems();
        let result_1 = AngleSolver::new(ShapeRes::new(shapes), false).find_problems();

        assert_eq!(result_0.is_empty(), true);
        assert_eq!(result_1.is_empty(), false);
    }

    #[test]
    fn test_5() {
        let shapes = [
            [
                [
                    IntPoint::new(-5, -15),
                    IntPoint::new(5, -15),
                    IntPoint::new(15, -5),
                    IntPoint::new(50, -5),
                    IntPoint::new(50, 5),
                    IntPoint::new(15, 5),
                    IntPoint::new(5, 15),
                    IntPoint::new(-5, 15),
                    IntPoint::new(-15, 5),
                    IntPoint::new(-50, 5),
                    IntPoint::new(-50, -5),
                    IntPoint::new(-15, -5)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let result_0 = AngleSolver::new(ShapeRes::new(shapes.clone()), true).find_problems();
        let result_1 = AngleSolver::new(ShapeRes::new(shapes), false).find_problems();

        assert_eq!(result_0.is_empty(), true);
        assert_eq!(result_1.is_empty(), false);
    }

    #[test]
    fn test_6() {
        let shapes = [
            [
                [
                    IntPoint::new(0, -10),
                    IntPoint::new(0, 10),
                    IntPoint::new(10, 0)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let result_0 = AngleSolver::new(ShapeRes::new(shapes.clone()), true).find_problems();
        let result_1 = AngleSolver::new(ShapeRes::new(shapes), false).find_problems();

        assert_eq!(result_0.is_empty(), false);
        assert_eq!(result_1.is_empty(), false);
    }

    #[test]
    fn test_7() {
        let shapes = [
            [
                [
                    IntPoint::new(0, 10),
                    IntPoint::new(0, -10),
                    IntPoint::new(10, 0)
                ].to_vec()
            ].to_vec()
        ].to_vec();

        let result_0 = AngleSolver::new(ShapeRes::new(shapes.clone()), true).find_problems();
        let result_1 = AngleSolver::new(ShapeRes::new(shapes), false).find_problems();

        assert_eq!(result_0.is_empty(), false);
        assert_eq!(result_1.is_empty(), false);
    }
}

