use std::collections::HashMap;
use foundation::fix::mark::Mark;
use foundation::i_overlay::i_shape::int::shape::IntShapes;
use foundation::i_overlay::id_point::IdPoint;
use crate::connectivity::search::ShapeSearch;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CellLabel {
    pub label: String,
    pub cell: String,
}

pub trait Marker {
    fn mark(&self, marks: &[Mark]) -> HashMap<CellLabel, Vec<usize>>;
}

impl Marker for IntShapes {
    fn mark(&self, marks: &[Mark]) -> HashMap<CellLabel, Vec<usize>> {
        if marks.is_empty() {
            return HashMap::new();
        }

        let mut id_points = Vec::with_capacity(marks.len());

        let mut id = 0;
        for mark in marks.iter() {
            id_points.push(IdPoint { id, point: mark.point });
            id += 1;
        }

        let pairs = self.search(&mut id_points);

        let mut map: HashMap<CellLabel, Vec<usize>> = HashMap::with_capacity(pairs.len());

        for pair in pairs.iter() {
            let mark = &marks[pair.point_id as usize];
            let key = CellLabel { label: mark.label.clone(), cell: mark.cell.clone() };
            let shape_id = pair.id as usize;
            if let Some(item) = map.get_mut(&key) {
                item.push(shape_id)
            } else {
                map.insert(key, vec![shape_id]);
            }
        }

        map
    }
}

#[cfg(test)]
mod tests {
    use foundation::fix::layer::LayerData;
    use foundation::fix::mark::Mark;
    use foundation::i_overlay::i_float::point::IntPoint;
    use crate::connectivity::marker::Marker;

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

        let result = shapes.mark(&[
            Mark { cell: "".to_string(), label: "22".to_string(), point: IntPoint::new(-10, -10), layer: LayerData { number: 0, datatype: 0 } },
        ]);

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_1() {
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

        let result = shapes.mark(&[
            Mark { cell: "".to_string(), label: "22".to_string(), point: IntPoint::new(-10, 0), layer: LayerData { number: 0, datatype: 0 } },
        ]);


        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_2() {
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

        let result = shapes.mark(&[
            Mark { cell: "".to_string(), label: "22".to_string(), point: IntPoint::new(-10, 10), layer: LayerData { number: 0, datatype: 0 } },
        ]);


        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_3() {
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

        let result = shapes.mark(&[
            Mark { cell: "".to_string(), label: "22".to_string(), point: IntPoint::new(0, -10), layer: LayerData { number: 0, datatype: 0 } },
        ]);


        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_4() {
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

        let result = shapes.mark(&[
            Mark { cell: "".to_string(), label: "22".to_string(), point: IntPoint::new(0, 0), layer: LayerData { number: 0, datatype: 0 } },
        ]);


        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_5() {
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

        let result = shapes.mark(&[
            Mark { cell: "".to_string(), label: "22".to_string(), point: IntPoint::new(0, 10), layer: LayerData { number: 0, datatype: 0 } },
        ]);


        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_6() {
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

        let result = shapes.mark(&[
            Mark { cell: "".to_string(), label: "22".to_string(), point: IntPoint::new(10, -10), layer: LayerData { number: 0, datatype: 0 } },
        ]);


        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_7() {
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

        let result = shapes.mark(&[
            Mark { cell: "".to_string(), label: "22".to_string(), point: IntPoint::new(10, 0), layer: LayerData { number: 0, datatype: 0 } },
        ]);


        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_8() {
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

        let result = shapes.mark(&[
            Mark { cell: "".to_string(), label: "22".to_string(), point: IntPoint::new(10, 10), layer: LayerData { number: 0, datatype: 0 } },
        ]);


        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_10() {
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

        let result = shapes.mark(&[
            Mark { cell: "".to_string(), label: "00".to_string(), point: IntPoint::new(-20, 20), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "01".to_string(), point: IntPoint::new(-10, 20), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "02".to_string(), point: IntPoint::new(0, 20), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "03".to_string(), point: IntPoint::new(10, 20), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "04".to_string(), point: IntPoint::new(20, 20), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "10".to_string(), point: IntPoint::new(-20, 10), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "11".to_string(), point: IntPoint::new(-10, 10), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "12".to_string(), point: IntPoint::new(0, 10), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "13".to_string(), point: IntPoint::new(10, 10), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "14".to_string(), point: IntPoint::new(20, 10), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "20".to_string(), point: IntPoint::new(-20, 0), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "21".to_string(), point: IntPoint::new(-10, 0), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "22".to_string(), point: IntPoint::new(0, 0), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "23".to_string(), point: IntPoint::new(10, 0), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "24".to_string(), point: IntPoint::new(20, 0), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "30".to_string(), point: IntPoint::new(-20, -10), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "31".to_string(), point: IntPoint::new(-10, -10), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "32".to_string(), point: IntPoint::new(0, -10), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "33".to_string(), point: IntPoint::new(10, -10), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "34".to_string(), point: IntPoint::new(20, -10), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "40".to_string(), point: IntPoint::new(-20, -20), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "41".to_string(), point: IntPoint::new(-10, -20), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "42".to_string(), point: IntPoint::new(0, -20), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "43".to_string(), point: IntPoint::new(10, -20), layer: LayerData { number: 0, datatype: 0 } },
            Mark { cell: "".to_string(), label: "44".to_string(), point: IntPoint::new(20, -20), layer: LayerData { number: 0, datatype: 0 } },
        ]);

        assert_eq!(result.len(), 9);
    }
}