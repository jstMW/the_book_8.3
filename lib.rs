use std::{collections::HashMap, hash::Hash};

//list of integers, use a vector and return the median
//(when sorted, the value in the middle position) and
//mode (the value that occurs most often; a hash map will be helpful here) of the list.

fn median(v: &Vec<usize>) -> f32 {
    let len: usize = v.len();
    if len%2==0 {
        (v[len/2 - 1] + v[len/2]) as f32 / 2.0
    } else {
        v[(len + 1) / 2 - 1] as f32
    }
    
}

fn mode(v: &Vec<usize>) -> usize {
    let mut mode: usize = 0;
    let mut hm: HashMap<usize, usize>  = HashMap::new();

    for element in v {
        let count = hm.entry(*element).or_insert(0);
        *count += 1;
    }

    for (element, rep) in hm {
        if rep>mode {
            mode = element;
        }
    }

    mode
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn median_one_element_panic() {
        let mut ints: Vec<usize> = Vec::new();
        ints.push(1);

        assert_eq!(median(&ints), 1.0);
    }

    #[test]
    fn median_odd_element_panic() {
        let mut ints: Vec<usize> = Vec::new();
        ints.push(1);
        ints.push(2);
        ints.push(3);

        assert_eq!(median(&ints), 2.0);
    }

    #[test]
    fn median_even_element_panic() {
        let mut ints: Vec<usize> = Vec::new();
        ints.push(1);
        ints.push(2);
        ints.push(3);
        ints.push(4);

        assert_eq!(median(&ints), 2.5);
    }


    #[test]
    fn mode_even_element_panic() {
        let mut ints: Vec<usize> = Vec::new();
        ints.push(1);
        ints.push(2);
        ints.push(4);
        ints.push(4);
        ints.push(4);
        ints.push(4);
        ints.push(4);
        ints.push(4);

        assert_eq!(mode(&ints), 4);
    }
    #[test]
    fn mode_odd_element_panic() {
        let mut ints: Vec<usize> = Vec::new();
        ints.push(1);
        ints.push(3);
        ints.push(3);

        assert_eq!(mode(&ints), 3);
    }
    #[test]
    fn mode_all_equal_panic() {
        let mut ints: Vec<usize> = Vec::new();
        ints.push(1);
        ints.push(2);
        ints.push(3);
        ints.push(4);

        assert_eq!(mode(&ints), 0); /*actually can't determine it for now!! */
    }
}
