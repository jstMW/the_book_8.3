use std::collections::HashMap;

fn median(v: &mut Vec<usize>) -> f32 {
    v.sort();
    let len: usize = v.len();
    match len {
        0 => f32::NAN,
        _ if len % 2 == 0 => (v[len / 2 - 1] + v[len / 2]) as f32 / 2.0,
        _ => v[len / 2] as f32,
    }
}

fn mode(v: &Vec<usize>) -> Option<usize> {
    let mut hm: HashMap<usize, usize> = HashMap::new();

    for element in v {
        let count = hm.entry(*element).or_insert(0);
        *count += 1;
    }

    hm.iter()
        .max_by_key(|&(key, val)| val)
        .map(|(&key, value)| key)
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn median_one_element() {
        let mut ints: Vec<usize> = Vec::new();
        ints.push(1);

        assert_eq!(median(&mut ints), 1.0);
    }

    #[test]
    fn median_odd_elements() {
        let mut ints: Vec<usize> = Vec::new();
        ints.push(1);
        ints.push(2);
        ints.push(3);

        assert_eq!(median(&mut ints), 2.0);
    }

    #[test]
    fn median_even_elements() {
        let mut ints: Vec<usize> = Vec::new();
        ints.push(1);
        ints.push(2);
        ints.push(3);
        ints.push(4);

        assert_eq!(median(&mut ints), 2.5);
    }
    #[test]
    fn median_unsorted_vec() {
        let mut ints: Vec<usize> = Vec::new();
        ints.push(1);
        ints.push(9);
        ints.push(0);
        ints.push(5);

        assert_eq!(median(&mut ints), 3.0);
    }

    #[test]
    fn median_empty_vec() {
        let mut ints: Vec<usize> = Vec::new();

        assert_eq!(median(&mut ints), f32::NAN);
    }
    #[test]
    fn mode_even_elements() {
        let mut ints: Vec<usize> = Vec::new();
        ints.push(1);
        ints.push(2);
        ints.push(4);
        ints.push(4);
        ints.push(4);
        ints.push(4);
        ints.push(4);
        ints.push(4);

        assert_eq!(mode(&ints), Some(4));
    }
    #[test]
    fn mode_odd_elements() {
        let mut ints: Vec<usize> = Vec::new();
        ints.push(1);
        ints.push(3);
        ints.push(3);

        assert_eq!(mode(&ints), Some(3));
    }
    #[test]
    fn mode_all_equal_return_None() {
        let mut ints: Vec<usize> = Vec::new();
        ints.push(1);
        ints.push(2);
        ints.push(3);
        ints.push(4);

        assert_eq!(mode(&ints), None); /*actually can't determine it for now!! */
    }

     #[test]
    fn mode_empty_vec() {
        let mut ints: Vec<usize> = Vec::new();

        assert_eq!(mode(&ints), None);
    }

}
