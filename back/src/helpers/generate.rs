use rand::random_range;

pub fn generate_random_string(length: i32) -> String {
    let possible_chars = "123456789abcdefghijklmnopqrstuvwxyz&#@^:!;.,$£*µ%§";
    let possible_chars_vec: Vec<char> = possible_chars.chars().collect();

    let mut result = String::new();

    for _ in 0..length {
        let random = random_range(0..possible_chars_vec.len());
        result.push(possible_chars_vec[random]);
    }

    return result;
}
