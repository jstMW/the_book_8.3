pub mod median;
pub mod mode;
pub mod pig_latin;
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

    pub mod pig_lating_tests {
        use crate::pig_latin::to_pig_latin;
        fn first_consonant_words() {
            assert_eq!(to_pig_latin("first"), "irst-fay");
            assert_eq!(to_pig_latin("second"), "econd-say");
            assert_eq!(to_pig_latin("third"), "hird-tay");
            assert_eq!(to_pig_latin("school"), "chool-say");
            assert_eq!(to_pig_latin("music"), "usic-may");
            assert_eq!(to_pig_latin("coffee"), "offee-cay");
        }

        fn first_wovel_words() {
            assert_eq!(to_pig_latin("apple"), "apple-hay");
            assert_eq!(to_pig_latin("ambient"), "ambient-hay");
            assert_eq!(to_pig_latin("air"), "air-hay");
            assert_eq!(to_pig_latin("ill"), "ill-hay");
            assert_eq!(to_pig_latin("erase"), "erase-hay");
            assert_eq!(to_pig_latin("after"), "after-hay");
        }

        fn upper_and_lower_case_words_mixed() {
            assert_eq!(to_pig_latin("IlL"), "ill-hay");
            assert_eq!(to_pig_latin("ERASER"), "erase-hay");
            assert_eq!(to_pig_latin("AfTer"), "after-hay");
            assert_eq!(to_pig_latin("school"), "chool-say");
            assert_eq!(to_pig_latin("Music"), "usic-may");
            assert_eq!(to_pig_latin("seCoNd"), "econd-say");
        }
    }
}
