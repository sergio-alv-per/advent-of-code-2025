#[macro_export]
macro_rules! test_with_files {
    ($day:literal, $solve_func:ident, $expected_sample:expr, $expected_complete:expr) => {
        #[test]
        fn test_sample() {
            let input = include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/inputs/",
                $day,
                "_sample.in"
            ));
            let result = $solve_func(&input);
            assert_eq!(result, $expected_sample);
        }

        #[test]
        fn test_complete() {
            let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/", $day, ".in"));
            let result = $solve_func(&input);
            assert_eq!(result, $expected_complete);
        }
    };
}
