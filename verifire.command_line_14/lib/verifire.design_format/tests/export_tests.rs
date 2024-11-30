#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use design_format::fix::doc::FixDoc;

    #[test]
    fn test_0() {
        let path_0 = "./tests/data/test_0/origin.gds";
        let path_1 = "./tests/data/test_0/raw.json";
        let path_2 = "./tests/data/test_0/back.gds";

        test_gds_to_raw_to_gds(path_0, path_1, path_2);
    }

    #[test]
    fn test_1() {
        let path_0 = "./tests/data/test_1/origin.gds";
        let path_1 = "./tests/data/test_1/raw.json";
        let path_2 = "./tests/data/test_1/back.gds";

        test_gds_to_raw_to_gds(path_0, path_1, path_2);
    }

    #[test]
    fn test_2() {
        let path_0 = "./tests/data/test_2/origin.gds";
        let path_1 = "./tests/data/test_2/raw.json";
        let path_2 = "./tests/data/test_2/back.gds";

        test_gds_to_raw_to_gds(path_0, path_1, path_2);
    }

    #[test]
    fn test_3() {
        let path_0 = "./tests/data/test_3/origin.gds";
        let path_1 = "./tests/data/test_3/raw.json";
        let path_2 = "./tests/data/test_3/back.gds";

        test_gds_to_raw_to_gds(path_0, path_1, path_2);
    }

    fn test_gds_to_raw_to_gds(path_0: &str, path_1: &str, path_2: &str) {
        let file_0 = Path::new(path_0);
        let file_1 = Path::new(path_1);
        let file_2 = Path::new(path_2);

        if file_1.exists() {
            fs::remove_file(file_1).expect("file not deleted");
        }

        if file_2.exists() {
            fs::remove_file(file_2).expect("file not deleted");
        }

        let doc_0 = FixDoc::with_gds_file(file_0).expect("can not open gds");
        doc_0.save_raw(file_1, true).expect("can not save raw");

        assert_eq!(file_1.exists(), true);

        let doc_1 = FixDoc::with_raw_file(file_1).expect("can not open raw");
        doc_1.save_gds(file_2).expect("can not save gds");

        assert_eq!(file_2.exists(), true);
    }
}