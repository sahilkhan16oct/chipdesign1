#[cfg(test)]
mod tests {
    use foundation::algorithm::slice::slice_type::SliceType;
    use foundation::i_overlay::i_float::point::IntPoint;
    use foundation::task::orientation::Orientation;
    use slice::solution::solver::SliceSolver;


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

        let result = SliceSolver::slice(shapes, Orientation::Vertical, SliceType::Original);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 4);
    }
}