pub mod median;
pub mod mode;
#[cfg(test)]
pub mod test {
    pub mod median_tests {
        use crate::median::median;
        #[test]
        fn median_one_element() {
            let mut ints: Vec<usize> = vec![1];

            assert_eq!(median(&mut ints), 1.0);
        }

        #[test]
        fn median_odd_elements() {
            let mut ints: Vec<usize> = vec![1, 2, 3];

            assert_eq!(median(&mut ints), 2.0);
        }

        #[test]
        fn median_even_elements() {
            let mut ints: Vec<usize> = vec![1, 2, 3, 4];

            assert_eq!(median(&mut ints), 2.5);
        }
        #[test]
        fn median_unsorted_vec() {
            let mut ints: Vec<usize> = vec![1, 9, 0, 5];

            assert_eq!(median(&mut ints), 3.0);
        }

        #[test]
        fn median_empty_vec() {
            let mut ints: Vec<usize> = Vec::new();

            assert!(median(&mut ints).is_nan());
        }
    }
    pub mod mode_tests {
        use crate::mode::mode;
        #[test]
        fn mode_even_elements() {
            let ints: Vec<usize> = vec![1, 2, 4, 4, 4, 4, 4, 4];
            assert_eq!(mode(&ints), Some(4));
        }
        #[test]
        fn mode_odd_elements() {
            let ints: Vec<usize> = vec![1, 2, 2];

            assert_eq!(mode(&ints), Some(2));
        }
        #[test]
        fn mode_all_equal_return_none() {
            let ints: Vec<usize> = vec![1, 2, 3, 4];

            assert_eq!(mode(&ints), None); /*actually can't determine it for now!! it will return the first I guess comparison*/
        }
        #[test]
        fn mode_multiple_equal_return_none() {
            let ints: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 4, 5, 5, 4];

            assert_eq!(mode(&ints), None); /*actually can't determine it for now!! it will return the first I guess comparison*/
        }
        #[test]
        fn mode_empty_vec() {
            let ints: Vec<usize> = Vec::new();

            assert_eq!(mode(&ints), None);
        }
    }
}
