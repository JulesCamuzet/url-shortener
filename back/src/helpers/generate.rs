use rand::random_range;

pub struct GenerateRandomStringOptions {
    pub include_uppercases: bool,
    pub include_specials: bool,
    pub include_numbers: bool
}

pub fn generate_random_string(
    length: i32,
    opts: GenerateRandomStringOptions
) -> String {
    let mut possible_chars = String::from("abcdefghijklmnopqrstuvwxyz");
    
    if opts.include_numbers == true {
        possible_chars.push_str("123456789");
    }
    
    if opts.include_uppercases == true {
        possible_chars.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    
    if opts.include_specials == true {
        possible_chars.push_str("&#@^:!;.,$£*µ%§");
    }
    
    let possible_chars_vec: Vec<char> = possible_chars.chars().collect();

    let mut result = String::new();

    for _ in 0..length {
        let random = random_range(0..possible_chars_vec.len());
        result.push(possible_chars_vec[random]);
    }

    return result;
}
