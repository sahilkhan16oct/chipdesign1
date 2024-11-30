use foundation::i_overlay::i_shape::int::path::{IntPath, PointPathExtension};
use foundation::i_overlay::i_shape::int::shape::IntShape;
use foundation::res::shape_res::ShapeRes;

pub struct MinAreaSolver {
    shapes_res: ShapeRes,
    min_area: i64,
}

impl MinAreaSolver {
    pub fn new(shapes_res: ShapeRes, min_area: i64) -> Self {
        Self {
            shapes_res,
            min_area,
        }
    }

    pub fn find_problems(&self) -> Vec<IntPath> {
        let shapes = self.shapes_res.shapes();

        let mut result = Vec::new();

        for shape in shapes.iter() {
            let area = shape.area();
            if self.min_area > area {
                result.push(shape[0].clone())
            }
        }

        result
    }
}

trait Area {
    fn area(&self) -> i64;
}

impl Area for IntShape {
    fn area(&self) -> i64 {
        let mut area = 0;
        for path in self.iter() {
            area += path.unsafe_area()
        }

        area >> 1
    }
}


#[cfg(test)]
mod tests {
    use foundation::i_overlay::i_float::point::IntPoint;
    use crate::solution::solver::Area;

    #[test]
    fn test_0() {
        let square =
            [
                [
                    IntPoint::new(-1024, -1024),
                    IntPoint::new(-1024, 1024),
                    IntPoint::new(1024, 1024),
                    IntPoint::new(1024, -1024)
                ].to_vec()
            ].to_vec();


        let area = square.area();

        assert_eq!(area, 4 * 1024 * 1024);
    }

    #[test]
    fn test_1() {
        let square = [
                [
                    IntPoint::new(-1024, -1024),
                    IntPoint::new(-1024, 1024),
                    IntPoint::new(1024, 1024),
                    IntPoint::new(1024, -1024)
                ].to_vec(),
                [
                    IntPoint::new(-512, -512),
                    IntPoint::new(512, -512),
                    IntPoint::new(512, 512),
                    IntPoint::new(-512, 512)
                ].to_vec()
            ].to_vec();

        let area = square.area();

        assert_eq!(area, 3 * 1024 * 1024);
    }
}