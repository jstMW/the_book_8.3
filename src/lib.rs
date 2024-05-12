use std::collections::HashMap;

fn median(v: &mut Vec<usize>) -> f32 {
    v.sort();
    let len: usize = v.len();
    match len {
        0 => f32::NAN,
        _ if len % 2 == 0 => (v[len / 2 - 1] + v[len / 2]) as f32 / 2.0,
        _ => v[(len + 1) / 2 - 1] as f32,
    }
}

fn mode(v: &Vec<usize>) -> usize {
    let mut mode: usize = 0;
    let mut hm: HashMap<usize, usize> = HashMap::new();

    for element in v {
        let count = hm.entry(*element).or_insert(0);
        *count += 1;
    }

    hm.iter()
        .max_by_key(|&(key, val)| val)
        .map(|(&key, value)| key)
        .unwrap()
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn median_one_element_panic() {
        let mut ints: Vec<usize> = Vec::new();
        ints.push(1);

        assert_eq!(median(&mut ints), 1.0);
    }

    #[test]
    fn median_odd_element_panic() {
        let mut ints: Vec<usize> = Vec::new();
        ints.push(1);
        ints.push(2);
        ints.push(3);

        assert_eq!(median(&mut ints), 2.0);
    }

    #[test]
    fn median_even_element_panic() {
        let mut ints: Vec<usize> = Vec::new();
        ints.push(1);
        ints.push(2);
        ints.push(3);
        ints.push(4);

        assert_eq!(median(&mut ints), 2.5);
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
