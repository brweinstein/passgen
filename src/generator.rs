use crate::random::random_idx;

pub fn create_password(length: usize, charset: &Vec<char>, no_repeat: bool) -> String {
    let mut result = String::with_capacity(length);
    let mut used = Vec::new();

    for _ in 0..length {
        let mut idx;
        loop {
            idx = random_idx(charset.len());
            if !no_repeat || !used.contains(&idx) {
                break;
            }
        }
        result.push(charset[idx]);
        used.push(idx);
    }

    result
}
