#[cfg(test)]
mod tests {
    use foundation::i_overlay::i_float::point::IntPoint;
    use foundation::space_extension::builder::SquareVectorBuilder;
    use foundation::space_extension::square_index::ShapeLayer;
    use foundation::space_extension::square_range::SquareRange;
    use foundation::space_extension::square_space::{SquareSpace, SquareSpaceBuffer};

    #[test]
    fn test_0() {
        let shapes = [
            [[
                IntPoint::new(-50, -50),
                IntPoint::new(-50, -10),
                IntPoint::new(-10, -10),
                IntPoint::new(-10, -50)
            ].to_vec()].to_vec(),
            [[
                IntPoint::new(-50, 10),
                IntPoint::new(-50, 50),
                IntPoint::new(-10, 50),
                IntPoint::new(-10, 10)
            ].to_vec()].to_vec(),
            [[
                IntPoint::new(10, 10),
                IntPoint::new(10, 50),
                IntPoint::new(50, 50),
                IntPoint::new(50, 10)
            ].to_vec()].to_vec(),
            [[
                IntPoint::new(10, -50),
                IntPoint::new(10, -10),
                IntPoint::new(50, -10),
                IntPoint::new(50, -50)
            ].to_vec()].to_vec()
        ].to_vec();

        let vectors = shapes.build_square_vectors(ShapeLayer::A, 0);
        let sq_space = SquareSpace::new(&vectors);


        let mut buffer = SquareSpaceBuffer::new();
        let mut ids: Vec<usize> = Vec::new();

        sq_space.ids_in_range(&SquareRange { x_min: -50, y_min: -50, x_max: -40, y_max: -40 }, usize::MAX, &mut buffer, &mut ids);
        assert_eq!(ids, [0, 1]);

        ids.clear();

        sq_space.ids_in_range(&SquareRange { x_min: -50, y_min: 40, x_max: -40, y_max: 50 }, usize::MAX, &mut buffer, &mut ids);
        assert_eq!(ids, [6, 5]);

        ids.clear();

        sq_space.ids_in_range(&SquareRange { x_min: 40, y_min: 40, x_max: 50, y_max: 50 }, usize::MAX, &mut buffer, &mut ids);
        assert_eq!(ids, [10, 11]);

        ids.clear();

        sq_space.ids_in_range(&SquareRange { x_min: 40, y_min: -50, x_max: 50, y_max: -40 }, usize::MAX, &mut buffer, &mut ids);
        assert_eq!(ids, [12, 15]);

        ids.clear();

        sq_space.ids_in_range(&SquareRange { x_min: -50, y_min: -50, x_max: 50, y_max: 50 }, usize::MAX, &mut buffer, &mut ids);
        ids.sort();
        assert_eq!(ids, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

        ids.clear();

        sq_space.ids_in_range(&SquareRange { x_min: -5, y_min: -5, x_max: 5, y_max: 5 }, usize::MAX, &mut buffer, &mut ids);
        assert_eq!(ids.len(), 0);

        ids.clear();

        sq_space.ids_in_range(&SquareRange { x_min: -10, y_min: -10, x_max: 10, y_max: 10 }, usize::MAX, &mut buffer, &mut ids);
        ids.sort();
        assert_eq!(ids, [2, 3, 4, 7, 8, 9, 13, 14]);
    }
}