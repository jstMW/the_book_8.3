use std::collections::HashMap;
pub fn mode(v: &[usize]) -> Option<usize> {
    let mut hm: HashMap<usize, usize> = HashMap::new();

    for element in v {
        let count = hm.entry(*element).or_insert(0);
        *count += 1;
    }

    hm.iter().max_by_key(|&(_, val)| val).map(|(&key, _)| key)
}
