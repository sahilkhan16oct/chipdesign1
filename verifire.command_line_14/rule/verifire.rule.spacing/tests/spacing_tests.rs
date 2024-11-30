#[cfg(test)]
mod tests {
    use foundation::i_overlay::i_float::fix_float::FixConvert;
    use foundation::i_overlay::i_float::point::IntPoint;
    use foundation::task::direction::SpaceDirection;
    use spacing::solution::builder::SpaceBuilder;
    use spacing::solution::solver::SpaceSolver;

    fn point(x: f64, y: f64) -> IntPoint {
        IntPoint::new((x * 1024.0) as i32, (y * 1024.0) as i32)
    }

    #[test]
    fn test_0() {
        let shape = [[
            point(-15.0, -5.0),
            point(-15.0, 5.0),
            point(-5.0, 5.0),
            point(-5.0, 15.0),
            point(5.0, 15.0),
            point(5.0, 5.0),
            point(15.0, 5.0),
            point(15.0, -5.0),
            point(5.0, -5.0),
            point(5.0, -15.0),
            point(-5.0, -15.0),
            point(-5.0, -5.0)
        ].to_vec()].to_vec();

        let shapes = [shape].to_vec();
        let errors = SpaceSolver::new(SpaceBuilder::new_shapes(shapes), 15.0.fix() as i32, SpaceDirection::DXY).find_problems();

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn test_1() {
        let shape0 = [[
            point(-5.0, 5.0),
            point(-5.0, 15.0),
            point(5.0, 15.0),
            point(5.0, 5.0)
        ].to_vec()].to_vec();

        let shape1 = [[
            point(-10.0, -15.0),
            point(-10.0, -5.0),
            point(10.0, -5.0),
            point(10.0, -15.0)
        ].to_vec()].to_vec();

        let shapes = [shape0, shape1].to_vec();

        assert_eq!(SpaceSolver::new(SpaceBuilder::new_shapes(shapes.clone()), 16.0.fix() as i32, SpaceDirection::DXY).find_problems().len(), 2);
        assert_eq!(SpaceSolver::new(SpaceBuilder::new_shapes(shapes.clone()), 20.0.fix() as i32, SpaceDirection::DXY).find_problems().len(), 2);
        assert_eq!(SpaceSolver::new(SpaceBuilder::new_shapes(shapes), 24.0.fix() as i32, SpaceDirection::DXY).find_problems().len(), 2);
    }
}