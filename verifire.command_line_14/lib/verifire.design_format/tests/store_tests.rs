#[cfg(test)]
mod tests {
    use std::path::Path;
    use i_overlay::i_float::point::IntPoint;
    use design_format::fix::doc::FixDoc;
    use design_format::fix::layer::LayerData;

    #[test]
    fn test_0_gds() {
        let file = Path::new("./tests/data/test_0/origin.gds");
        let mut doc = FixDoc::with_gds_file(file).expect("can not open gds");

        test_0(&mut doc);
    }

    #[test]
    fn test_0_raw() {
        let file = Path::new("./tests/data/test_0/origin.json");
        let mut doc = FixDoc::with_raw_file(file).expect("can not open raw");

        test_0(&mut doc);
    }

    fn test_0(doc: &mut FixDoc) {
        let layer = LayerData { number: 0, datatype: 0 };

        let store = doc.cell_shape_store(&"MAIN".to_string(), &vec![layer]).expect("Bad data");

        let item = store.get(&layer);

        assert_eq!(item.unwrap().len(), 6);
        assert!(!store.contains_key(&LayerData { number: 1, datatype: 0 }));
    }

    #[test]
    fn test_1_gds() {
        let file = Path::new("./tests/data/test_1/origin.gds");
        let mut doc = FixDoc::with_gds_file(file).expect("can not open gds");
        test_1(&mut doc);
    }

    #[test]
    fn test_1_raw() {
        let file = Path::new("./tests/data/test_1/origin.json");
        let mut doc = FixDoc::with_raw_file(file).expect("can not open raw");
        test_1(&mut doc);
    }

    fn test_1(doc: &mut FixDoc) {
        let layers = vec![
            LayerData { number: 1, datatype: 0 },
            LayerData { number: 2, datatype: 0 },
            LayerData { number: 3, datatype: 0 },
            LayerData { number: 4, datatype: 0 },
            LayerData { number: 5, datatype: 0 },
            LayerData { number: 6, datatype: 0 },
            LayerData { number: 7, datatype: 0 },
            LayerData { number: 8, datatype: 0 },
            LayerData { number: 9, datatype: 0 },
            LayerData { number: 10, datatype: 0 },
            LayerData { number: 11, datatype: 0 },
        ];

        let store = doc.cell_shape_store(&"xr03d7".to_string(), &layers).expect("Bad data");

        for layer in layers.iter() {
            assert!(store.contains_key(layer),"index: {}", layer.number);
        }

        assert!(!store.contains_key(&LayerData { number: 0, datatype: 0 }));
        assert!(!store.contains_key(&LayerData { number: 12, datatype: 0 }));
    }

    #[test]
    fn test_2_gds() {
        let file = Path::new("./tests/data/test_2/origin.gds");
        let mut doc = FixDoc::with_gds_file(file).expect("can not open gds");
        test_2(&mut doc);
    }

    #[test]
    fn test_2_raw() {
        let file = Path::new("./tests/data/test_2/origin.json");
        let mut doc = FixDoc::with_raw_file(file).expect("can not open raw");
        test_2(&mut doc);
    }

    fn test_2(doc: &mut FixDoc) {

        let layer_00 = LayerData { number: 0, datatype: 0 };

        let layer_10 = LayerData { number: 1, datatype: 0 };
        let layer_11 = LayerData { number: 1, datatype: 1 };
        let layer_12 = LayerData { number: 1, datatype: 2 };
        let layer_13 = LayerData { number: 1, datatype: 3 };
        let layer_14 = LayerData { number: 1, datatype: 4 };

        let layer_20 = LayerData { number: 2, datatype: 0 };
        let layer_21 = LayerData { number: 2, datatype: 1 };
        let layer_22 = LayerData { number: 2, datatype: 2 };
        let layer_23 = LayerData { number: 2, datatype: 3 };
        let layer_24 = LayerData { number: 2, datatype: 4 };

        let layers = vec![
            layer_00,
            layer_10,
            layer_11,
            layer_12,
            layer_13,
            layer_14,
            layer_20,
            layer_21,
            layer_22,
            layer_23,
            layer_24,
        ];

        let store = doc.cell_shape_store(&"MAIN".to_string(), &layers).expect("Bad data");

        let item_00 = store.get(&layer_00).unwrap();

        assert_eq!(item_00.len(), 3);

        // 10

        let item = store.get(&layer_10).unwrap();
        assert_eq!(item.len(), 1);

        let path = vec![
            IntPoint::new(5000, 35000),
            IntPoint::new(5000, 45000),
            IntPoint::new(15000, 45000),
            IntPoint::new(15000, 40000),
            IntPoint::new(10000, 40000),
            IntPoint::new(10000, 35000),
        ];

        assert_eq!(item[0][0], path);

        // 11

        let item = store.get(&layer_11).unwrap();
        assert_eq!(item.len(), 1);

        let path = vec![
            IntPoint::new(25000, 35000),
            IntPoint::new(25000, 45000),
            IntPoint::new(30000, 45000),
            IntPoint::new(30000, 40000),
            IntPoint::new(35000, 40000),
            IntPoint::new(35000, 35000),
        ];

        assert_eq!(item[0][0], path);

        // 12

        let item = store.get(&layer_12).unwrap();
        assert_eq!(item.len(), 1);

        let path = vec![
            IntPoint::new(45000, 35000),
            IntPoint::new(45000, 40000),
            IntPoint::new(50000, 40000),
            IntPoint::new(50000, 45000),
            IntPoint::new(55000, 45000),
            IntPoint::new(55000, 35000),
        ];

        assert_eq!(item[0][0], path);

        // 13

        let item = store.get(&layer_13).unwrap();
        assert_eq!(item.len(), 1);

        let path = vec![
            IntPoint::new(65000, 40000),
            IntPoint::new(65000, 45000),
            IntPoint::new(75000, 45000),
            IntPoint::new(75000, 35000),
            IntPoint::new(70000, 35000),
            IntPoint::new(70000, 40000),
        ];

        assert_eq!(item[0][0], path);

        // 14

        let item = store.get(&layer_14).unwrap();
        assert_eq!(item.len(), 1);

        let path = vec![
            IntPoint::new(84208, 44056),
            IntPoint::new(94056, 45792),
            IntPoint::new(94924, 40868),
            IntPoint::new(90000, 40000),
            IntPoint::new(90868, 35076),
            IntPoint::new(85944, 34208),
        ];

        assert_eq!(item[0][0], path);

        // 20

        let item = store.get(&layer_20).unwrap();
        assert_eq!(item.len(), 1);

        let path = vec![
            IntPoint::new(5000, 55000),
            IntPoint::new(5000, 65000),
            IntPoint::new(10000, 65000),
            IntPoint::new(10000, 60000),
            IntPoint::new(15000, 60000),
            IntPoint::new(15000, 55000),
        ];

        assert_eq!(item[0][0], path);

        // 21

        let item = store.get(&layer_21).unwrap();
        assert_eq!(item.len(), 1);

        let path = vec![
            IntPoint::new(25000, 55000),
            IntPoint::new(25000, 60000),
            IntPoint::new(30000, 60000),
            IntPoint::new(30000, 65000),
            IntPoint::new(35000, 65000),
            IntPoint::new(35000, 55000),
        ];

        assert_eq!(item[0][0], path);

        // 22

        let item = store.get(&layer_22).unwrap();
        assert_eq!(item.len(), 1);

        let path = vec![
            IntPoint::new(45000, 60000),
            IntPoint::new(45000, 65000),
            IntPoint::new(55000, 65000),
            IntPoint::new(55000, 55000),
            IntPoint::new(50000, 55000),
            IntPoint::new(50000, 60000),
        ];

        assert_eq!(item[0][0], path);

        // 23

        let item = store.get(&layer_23).unwrap();
        assert_eq!(item.len(), 1);

        let path = vec![
            IntPoint::new(65000, 55000),
            IntPoint::new(65000, 65000),
            IntPoint::new(75000, 65000),
            IntPoint::new(75000, 60000),
            IntPoint::new(70000, 60000),
            IntPoint::new(70000, 55000),
        ];

        assert_eq!(item[0][0], path);

        // 24

        let item = store.get(&layer_24).unwrap();
        assert_eq!(item.len(), 1);

        let path = vec![
            IntPoint::new(84208, 64056),
            IntPoint::new(89132, 64924),
            IntPoint::new(90000, 60000),
            IntPoint::new(94924, 60868),
            IntPoint::new(95792, 55944),
            IntPoint::new(85944, 54208),
        ];

        assert_eq!(item[0][0], path);
    }
}