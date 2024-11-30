#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use foundation::i_overlay::i_float::point::IntPoint;
    use foundation::task::orientation::Orientation;
    use min_max_width::solution::edge::Edge;
    use min_max_width::solution::min_max_width::MinMaxWidth;
    use min_max_width::solution::solver::Condition;

    #[test]
    fn test_0() {
        let shape = [
            [
                IntPoint::new(-15, -15),
                IntPoint::new(-15, 15),
                IntPoint::new(15, 15),
                IntPoint::new(15, -15)
            ].to_vec()].to_vec();


        assert!(shape.find_xy_problems(Orientation::Vertical, Condition::Min(30)).is_empty());
        assert!(shape.find_xy_problems(Orientation::Vertical, Condition::Max(35)).is_empty());
        let set_0: HashSet<Edge> = shape.find_xy_problems(Orientation::Vertical, Condition::Min(35)).into_iter().collect();
        let set_1: HashSet<Edge> = shape.find_xy_problems(Orientation::Vertical, Condition::Max(25)).into_iter().collect();
        let set_template = HashSet::from([
            Edge::with_array([-15, -15, -15, 15]),
            Edge::with_array([ 15, -15, 15, 15])
        ]);

        assert_eq!(set_0, set_template);
        assert_eq!(set_1, set_template);
    }

    #[test]
    fn test_1() {
        let shape = [
            [
                IntPoint::new(-15, -15),
                IntPoint::new(-15, 15),
                IntPoint::new(15, 15),
                IntPoint::new(15, -15)
            ].to_vec()].to_vec();


        assert!(shape.find_xy_problems(Orientation::Horizontal, Condition::Min(30)).is_empty());
        assert!(shape.find_xy_problems(Orientation::Horizontal, Condition::Max(35)).is_empty());
        let set_0: HashSet<Edge> = shape.find_xy_problems(Orientation::Horizontal, Condition::Min(35)).into_iter().collect();
        let set_1: HashSet<Edge> = shape.find_xy_problems(Orientation::Horizontal, Condition::Max(25)).into_iter().collect();
        let set_template = HashSet::from([
            Edge::with_array([-15, -15, 15, -15]),
            Edge::with_array([-15,  15, 15, 15])
        ]);

        assert_eq!(set_0, set_template);
        assert_eq!(set_1, set_template);
    }

    #[test]
    fn test_2() {
        let shape = [
            [
                IntPoint::new(-5, -5),
                IntPoint::new(-15, -5),
                IntPoint::new(-15, 5),
                IntPoint::new(-5,  5),
                IntPoint::new(-5, 15),
                IntPoint::new(5, 15),
                IntPoint::new(5, 5),
                IntPoint::new(15, 5),
                IntPoint::new(15,-5),
                IntPoint::new(5,-5),
                IntPoint::new(5,-15),
                IntPoint::new(-5,-15)
            ].to_vec()].to_vec();


        assert!(shape.find_xy_problems(Orientation::Vertical, Condition::Min(5)).is_empty());
        let set_0: HashSet<Edge> = shape.find_xy_problems(Orientation::Vertical, Condition::Min(15)).into_iter().collect();

        let set_template = HashSet::from([
            Edge::with_array([-15, -5, -15, 5]),
            Edge::with_array([ -5, -5, -5,  5]),
            Edge::with_array([  5, -5,  5,  5]),
            Edge::with_array([ 15, -5, 15,  5])
        ]);

        assert_eq!(set_0, set_template);
    }
}