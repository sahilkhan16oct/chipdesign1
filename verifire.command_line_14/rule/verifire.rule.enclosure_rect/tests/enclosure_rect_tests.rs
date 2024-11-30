#[cfg(test)]
mod tests {
    use enclosure_rect::solution::solver::EnclosureRectSolver;
    use foundation::i_overlay::i_float::point::IntPoint;
    use foundation::i_overlay::i_shape::int::path::IntPath;
    use foundation::i_overlay::i_shape::int::shape::IntShape;
    use foundation::res::shape_res::ShapeRes;

    fn point(x: f64, y: f64) -> IntPoint {
        IntPoint::new((x * 1024.0).round() as i32, (y * 1024.0).round() as i32)
    }

    #[test]
    fn test_0() {
        let float_shapes = [[[
            point(-20.0, -20.0),
            point(-20.0, 20.0),
            point(20.0, 20.0),
            point(20.0, -20.0)
        ].to_vec()].to_vec()].to_vec();

        let rects_shapes = [
            [[
                point(-15.0, 5.0),
                point(-15.0, 15.0),
                point(-5.0, 15.0),
                point(-5.0, 5.0)
            ].to_vec()].to_vec(),
            [[
                point(-15.0, -15.0),
                point(-15.0, -5.0),
                point(-5.0, -5.0),
                point(-5.0, -15.0)
            ].to_vec()].to_vec(),
            [[
                point(5.0, 5.0),
                point(5.0, 15.0),
                point(15.0, 15.0),
                point(15.0, 5.0)
            ].to_vec()].to_vec(),
            [[
                point(5.0, -15.0),
                point(5.0, -5.0),
                point(15.0, -5.0),
                point(15.0, -15.0)
            ].to_vec()].to_vec(),
        ].to_vec();

        let any = "left_0 || right_0 || top_0 || bottom_0";
        let all = "left_0 && right_0 && top_0 && bottom_0";
        let left = "left_0";
        let right = "right_0";
        let top = "top_0";
        let bottom = "bottom_0";

        let result_0_any = execute(any, float_shapes.clone(), rects_shapes.clone(), -2048, None, None);
        let result_0_all = execute(all, float_shapes.clone(), rects_shapes.clone(), -2048, None, None);

        let result_1_any = execute(any, float_shapes.clone(), rects_shapes.clone(), 0, None, None);
        let result_1_all = execute(all, float_shapes.clone(), rects_shapes.clone(), 0, None, None);

        let result_2_any = execute(any, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);
        let result_2_all = execute(all, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);

        let result_3_any = execute(any, float_shapes.clone(), rects_shapes.clone(), 10240, None, None);
        let result_3_all = execute(all, float_shapes.clone(), rects_shapes.clone(), 10240, None, None);
        let result_3_left = execute(left, float_shapes.clone(), rects_shapes.clone(), 10240, None, None);
        let result_3_right = execute(right, float_shapes.clone(), rects_shapes.clone(), 10240, None, None);
        let result_3_top = execute(top, float_shapes.clone(), rects_shapes.clone(), 10240, None, None);
        let result_3_bottom = execute(bottom, float_shapes.clone(), rects_shapes.clone(), 10240, None, None);

        assert_eq!(errors_count(result_0_any), 0);
        assert_eq!(errors_count(result_0_all), 0);

        assert_eq!(errors_count(result_1_any), 0);
        assert_eq!(errors_count(result_1_all), 0);

        assert_eq!(errors_count(result_2_any), 0);
        assert_eq!(errors_count(result_2_all), 0);

        assert_eq!(errors_count(result_3_any), 0);
        assert_eq!(errors_count(result_3_all), 4);
        assert_eq!(errors_count(result_3_left), 2);
        assert_eq!(errors_count(result_3_right), 2);
        assert_eq!(errors_count(result_3_top), 2);
        assert_eq!(errors_count(result_3_bottom), 2);
    }

    #[test]
    fn test_1() {
        let float_shapes = [[[
            point(-20.0, -20.0),
            point(-20.0, 20.0),
            point(20.0, 20.0),
            point(20.0, -20.0)
        ].to_vec()].to_vec()].to_vec();

        let rects_shapes = [
            [[
                point(-20.0, 5.0),
                point(-20.0, 15.0),
                point(-10.0, 15.0),
                point(-10.0, 5.0)
            ].to_vec()].to_vec(),
            [[
                point(-15.0, -20.0),
                point(-15.0, -10.0),
                point(-5.0, -10.0),
                point(-5.0, -20.0)
            ].to_vec()].to_vec(),
            [[
                point(5.0, 10.0),
                point(5.0, 20.0),
                point(15.0, 20.0),
                point(15.0, 10.0)
            ].to_vec()].to_vec(),
            [[
                point(10.0, -15.0),
                point(10.0, -5.0),
                point(20.0, -5.0),
                point(20.0, -15.0)
            ].to_vec()].to_vec(),
        ].to_vec();

        let any = "left_0 || right_0 || top_0 || bottom_0";
        let all = "left_0 && right_0 && top_0 && bottom_0";
        let left = "left_0";
        let right = "right_0";
        let top = "top_0";
        let bottom = "bottom_0";

        let result_0_any = execute(any, float_shapes.clone(), rects_shapes.clone(), -2048, None, None);
        let result_0_all = execute(all, float_shapes.clone(), rects_shapes.clone(), -2048, None, None);

        let result_1_any = execute(any, float_shapes.clone(), rects_shapes.clone(), 0, None, None);
        let result_1_all = execute(all, float_shapes.clone(), rects_shapes.clone(), 0, None, None);

        let result_2_any = execute(any, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);
        let result_2_all = execute(all, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);
        let result_2_left = execute(left, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);
        let result_2_right = execute(right, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);
        let result_2_top = execute(top, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);
        let result_2_bottom = execute(bottom, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);

        assert_eq!(errors_count(result_0_any), 0);
        assert_eq!(errors_count(result_0_all), 0);

        assert_eq!(errors_count(result_1_any), 0);
        assert_eq!(errors_count(result_1_all), 0);

        assert_eq!(errors_count(result_2_any), 0);
        assert_eq!(errors_count(result_2_all), 4);
        assert_eq!(errors_count(result_2_left), 1);
        assert_eq!(errors_count(result_2_right), 1);
        assert_eq!(errors_count(result_2_top), 1);
        assert_eq!(errors_count(result_2_bottom), 1);
    }

    #[test]
    fn test_2() {
        let float_shapes = [[[
            point(-20.0, -20.0),
            point(-20.0, 20.0),
            point(20.0, 20.0),
            point(20.0, -20.0)
        ].to_vec()].to_vec()].to_vec();

        let rects_shapes = [
            [[
                point(-25.0, 5.0),
                point(-25.0, 15.0),
                point(-15.0, 15.0),
                point(-15.0, 5.0)
            ].to_vec()].to_vec(),
            [[
                point(-15.0, -25.0),
                point(-15.0, -15.0),
                point(-5.0, -15.0),
                point(-5.0, -25.0)
            ].to_vec()].to_vec(),
            [[
                point(5.0, 15.0),
                point(5.0, 25.0),
                point(15.0, 25.0),
                point(15.0, 15.0)
            ].to_vec()].to_vec(),
            [[
                point(15.0, -15.0),
                point(15.0, -5.0),
                point(25.0, -5.0),
                point(25.0, -15.0)
            ].to_vec()].to_vec(),
        ].to_vec();

        let any = "left_0 || right_0 || top_0 || bottom_0";
        let all = "left_0 && right_0 && top_0 && bottom_0";
        let left = "left_0";
        let right = "right_0";
        let top = "top_0";
        let bottom = "bottom_0";

        let result_0_any = execute(any, float_shapes.clone(), rects_shapes.clone(), -2048, None, None);
        let result_0_all = execute(all, float_shapes.clone(), rects_shapes.clone(), -2048, None, None);

        let result_1_any = execute(any, float_shapes.clone(), rects_shapes.clone(), 0, None, None);
        let result_1_all = execute(all, float_shapes.clone(), rects_shapes.clone(), 0, None, None);

        let result_2_any = execute(any, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);
        let result_2_all = execute(all, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);
        let result_2_left = execute(left, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);
        let result_2_right = execute(right, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);
        let result_2_top = execute(top, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);
        let result_2_bottom = execute(bottom, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);

        assert_eq!(errors_count(result_0_any), 0);
        assert_eq!(errors_count(result_0_all), 4);

        assert_eq!(errors_count(result_1_any), 0);
        assert_eq!(errors_count(result_1_all), 4);

        assert_eq!(errors_count(result_2_any), 0);
        assert_eq!(errors_count(result_2_all), 4);
        assert_eq!(errors_count(result_2_left), 3);
        assert_eq!(errors_count(result_2_right), 3);
        assert_eq!(errors_count(result_2_top), 3);
        assert_eq!(errors_count(result_2_bottom), 3);
    }

    #[test]
    fn test_3() {
        let float_shapes = [[[
            point(-15.0, -20.0),
            point(-15.0, -10.0),
            point(-20.0, -10.0),
            point(-20.0, 15.0),
            point(-10.0, 15.0),
            point(-10.0, 20.0),
            point(15.0, 20.0),
            point(15.0, 10.0),
            point(20.0, 10.0),
            point(20.0, -15.0),
            point(10.0, -15.0),
            point(10.0, -20.0)
        ].to_vec()].to_vec()].to_vec();

        let rects_shapes = [
            [[
                point(-15.0, 10.0),
                point(-15.0, 20.0),
                point(-5.0, 20.0),
                point(-5.0, 10.0)
            ].to_vec()].to_vec(),
            [[
                point(-20.0, -15.0),
                point(-20.0, -5.0),
                point(-10.0, -5.0),
                point(-10.0, -15.0)
            ].to_vec()].to_vec(),
            [[
                point(10.0, 5.0),
                point(10.0, 15.0),
                point(20.0, 15.0),
                point(20.0, 5.0)
            ].to_vec()].to_vec(),
            [[
                point(5.0, -20.0),
                point(5.0, -10.0),
                point(15.0, -10.0),
                point(15.0, -20.0)
            ].to_vec()].to_vec(),
        ].to_vec();

        let any = "left_0 || right_0 || top_0 || bottom_0";
        let all = "left_0 && right_0 && top_0 && bottom_0";
        let left = "left_0";
        let right = "right_0";
        let top = "top_0";
        let bottom = "bottom_0";

        let result_0_any = execute(any, float_shapes.clone(), rects_shapes.clone(), -2048, None, None);
        let result_0_all = execute(all, float_shapes.clone(), rects_shapes.clone(), -2048, None, None);

        let result_1_any = execute(any, float_shapes.clone(), rects_shapes.clone(), 0, None, None);
        let result_1_all = execute(all, float_shapes.clone(), rects_shapes.clone(), 0, None, None);

        let result_2_any = execute(any, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);
        let result_2_all = execute(all, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);
        let result_2_left = execute(left, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);
        let result_2_right = execute(right, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);
        let result_2_top = execute(top, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);
        let result_2_bottom = execute(bottom, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);

        assert_eq!(errors_count(result_0_any), 0);
        assert_eq!(errors_count(result_0_all), 4);

        assert_eq!(errors_count(result_1_any), 0);
        assert_eq!(errors_count(result_1_all), 4);

        assert_eq!(errors_count(result_2_any), 0);
        assert_eq!(errors_count(result_2_all), 4);
        assert_eq!(errors_count(result_2_left), 2);
        assert_eq!(errors_count(result_2_right), 2);
        assert_eq!(errors_count(result_2_top), 2);
        assert_eq!(errors_count(result_2_bottom), 2);
    }

    #[test]
    fn test_4() {
        let float_shapes = [[[
            point(-15.0, -20.0),
            point(-15.0, -10.0),
            point(-20.0, -10.0),
            point(-20.0, 15.0),
            point(-10.0, 15.0),
            point(-10.0, 20.0),
            point(15.0, 20.0),
            point(15.0, 10.0),
            point(20.0, 10.0),
            point(20.0, -15.0),
            point(10.0, -15.0),
            point(10.0, -20.0)
        ].to_vec()].to_vec()].to_vec();

        let rects_shapes = [
            [[
                point(-15.0, 15.0),
                point(-15.0, 25.0),
                point(-5.0, 25.0),
                point(-5.0, 15.0)
            ].to_vec()].to_vec(),
            [[
                point(-25.0, -15.0),
                point(-25.0, -5.0),
                point(-15.0, -5.0),
                point(-15.0, -15.0)
            ].to_vec()].to_vec(),
            [[
                point(15.0, 5.0),
                point(15.0, 15.0),
                point(25.0, 15.0),
                point(25.0, 5.0)
            ].to_vec()].to_vec(),
            [[
                point(5.0, -25.0),
                point(5.0, -15.0),
                point(15.0, -15.0),
                point(15.0, -25.0)
            ].to_vec()].to_vec(),
        ].to_vec();

        let any = "left_0 || right_0 || top_0 || bottom_0";
        let all = "left_0 && right_0 && top_0 && bottom_0";
        let left = "left_0";
        let right = "right_0";
        let top = "top_0";
        let bottom = "bottom_0";

        let result_0_any = execute(any, float_shapes.clone(), rects_shapes.clone(), -2048, None, None);
        let result_0_all = execute(all, float_shapes.clone(), rects_shapes.clone(), -2048, None, None);

        let result_1_any = execute(any, float_shapes.clone(), rects_shapes.clone(), 0, None, None);
        let result_1_all = execute(all, float_shapes.clone(), rects_shapes.clone(), 0, None, None);

        let result_2_any = execute(any, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);
        let result_2_all = execute(all, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);
        let result_2_left = execute(left, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);
        let result_2_right = execute(right, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);
        let result_2_top = execute(top, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);
        let result_2_bottom = execute(bottom, float_shapes.clone(), rects_shapes.clone(), 5120, None, None);

        assert_eq!(errors_count(result_0_any), 4);
        assert_eq!(errors_count(result_0_all), 4);

        assert_eq!(errors_count(result_1_any), 4);
        assert_eq!(errors_count(result_1_all), 4);

        assert_eq!(errors_count(result_2_any), 0);
        assert_eq!(errors_count(result_2_all), 4);
        assert_eq!(errors_count(result_2_left), 3);
        assert_eq!(errors_count(result_2_right), 3);
        assert_eq!(errors_count(result_2_top), 3);
        assert_eq!(errors_count(result_2_bottom), 3);
    }

    fn execute(expression: &str,
               float_shapes: Vec<IntShape>,
               rects_shapes: Vec<IntShape>,
               value_0: i32,
               value_1: Option<i32>,
               only_inner: Option<bool>,
    ) -> Option<Vec<IntPath>> {
        match EnclosureRectSolver::new(
            expression,
            ShapeRes::new(float_shapes),
            ShapeRes::new(rects_shapes),
            value_0,
            value_1,
            only_inner,
        ) {
            Ok(solver) => {
                Some(solver.find_problems())
            }
            Err(error) => {
                panic!("{}", error);
            }
        }
    }

    fn errors_count(result: Option<Vec<IntPath>>) -> usize {
        if let Some(errors) = result {
            errors.len()
        } else {
            usize::MAX
        }
    }
}
