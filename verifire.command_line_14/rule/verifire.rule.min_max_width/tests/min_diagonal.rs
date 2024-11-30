#[cfg(test)]
mod tests {
    use foundation::i_overlay::core::fill_rule::FillRule;
    use foundation::i_overlay::core::overlay_rule::OverlayRule;
    use foundation::i_overlay::core::overlay::{Overlay, ShapeType};
    use foundation::i_overlay::i_float::point::IntPoint;
    use foundation::i_overlay::i_shape::int::path::IntPath;
    use foundation::i_overlay::i_shape::int::shape::IntShape;
    use foundation::support::fix::ToFixShape;
    use min_max_width::solution::min_diagonal::MinDiagonal;

    #[test]
    fn test_0() {
        let shape = [
            [
                IntPoint::new(-10240, -10240),
                IntPoint::new(-10240, 10240),
                IntPoint::new(10240, 10240),
                IntPoint::new(10240, -10240)
            ].to_vec()
        ].to_vec().to_fix();

        assert_eq!(shape.find_diagonal_problems(10240).len(), 0);
        assert_eq!(shape.find_diagonal_problems(20500).len(), 0);
        assert_eq!(shape.find_diagonal_problems(40000).len(), 2);
    }

    #[test]
    fn test_1() {
        let shape = [
            [
                IntPoint::new(-20480, -20480),
                IntPoint::new(-20480, 20480),
                IntPoint::new(20480, 20480),
                IntPoint::new(20480, -20480)
            ].to_vec(),
            [
                IntPoint::new(10240, -10240),
                IntPoint::new(10240, 10240),
                IntPoint::new(-10240, 10240),
                IntPoint::new(-10240, -10240)
            ].to_vec()
        ].to_vec().to_fix();

        assert_eq!(shape.find_diagonal_problems(12325).len(), 8);
        assert_eq!(shape.find_diagonal_problems(15618).len(), 12);
    }

    #[test]
    fn test_2() {
        let shape = many_squares_union().to_fix();

        assert_eq!(shape.find_diagonal_problems(10240).len(), 208);
    }

    fn many_squares_union() -> IntShape {
        let n: i32 = 5;

        let size: i32 = 20000 / 2;
        let offset: i32 = 30000 / 2;


        let x = -((n - 1) * offset + size) / 2;
        let start = IntPoint::new(x, x);

        let subj_paths = many_squares(
            IntPoint::new(x, x),
            size,
            offset,
            n as usize,
        );

        let clip_paths = many_squares(
            IntPoint::new(start.x + offset / 2, start.y + offset / 2),
            size,
            offset,
            (n - 1) as usize,
        );

        let mut overlay = Overlay::new(2);
        overlay.add_paths(&subj_paths, ShapeType::Subject);
        overlay.add_paths(&clip_paths, ShapeType::Subject);

        let graph = overlay.into_graph(FillRule::NonZero);

        let union = graph.extract_shapes(OverlayRule::Subject);

        union[0].clone()
    }

    fn many_squares(start: IntPoint, size: i32, offset: i32, n: usize) -> Vec<IntPath> {
        let mut result = Vec::with_capacity(n * n);
        let mut y = start.y;
        for _ in 0..n {
            let mut x = start.x;
            for _ in 0..n {
                let path: IntPath = vec![
                    IntPoint::new(x, y),
                    IntPoint::new(x, y + size),
                    IntPoint::new(x + size, y + size),
                    IntPoint::new(x + size, y),
                ];
                result.push(path);
                x += offset;
            }
            y += offset;
        }

        result
    }
}