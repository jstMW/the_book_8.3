pub const wovels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
pub fn to_pig_latin(s: &str) -> &str {
    let is_wovel: bool = is_first_wovel(s);
    let new_s = s.clone().to_string().push_str("-hay");
    if is_wovel {
        return make_first_wovel(s);
    } else {
    }
    s
}

fn is_first_wovel(word: &str) -> bool {
    let first_letter = word.chars().next().unwrap();

    for letter in wovels {
        if first_letter == letter {
            return true
        }
    }

    false
}

fn make_first_wovel(s: &str) -> &str {
    
}