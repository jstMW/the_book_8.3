pub fn median(v: &mut Vec<usize>) -> f32 {
    v.sort();
    let len: usize = v.len();
    match len {
        0 => f32::NAN,
        _ if len % 2 == 0 => (v[len / 2 - 1] + v[len / 2]) as f32 / 2.0,
        _ => v[len / 2] as f32,
    }
}
