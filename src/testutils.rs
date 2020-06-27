
pub fn test_distance_func<ResultType: PartialEq + std::fmt::Debug + std::str::FromStr>(filename : &str, func: fn(&str, &str) -> ResultType) {
    let mut reader = csv::ReaderBuilder::new().has_headers(false).from_path(filename).unwrap();
    let mut num_tested = 0;
    for result in reader.records() {
        let rec = result.unwrap();
        let expected : ResultType = rec[2].parse().ok().unwrap();
        assert_eq!(func(&rec[0], &rec[1]), expected);
        num_tested += 1;
    }
    assert!(num_tested > 0);
}

