use regex::{Regex, Error};

pub fn check_email_format(val: &str) -> Result<bool, Error> {
    let regex = Regex::new(r"(?i)^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$")?;
    Ok(regex.is_match(val))
}

pub fn check_password_format(val: &str) -> bool {
    let mut has_whitespace = false;
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;

    for c in val.chars() {
        has_whitespace |= c.is_whitespace();
        has_lower |= c.is_lowercase();
        has_upper |= c.is_uppercase();
        has_digit |= c.is_digit(10);
    }

    !has_whitespace && has_upper && has_lower && has_digit && val.len() >= 8
}
