pub fn is_number(x: &char) -> bool {
    //*x >= '0' && *x <= '9'
    x.is_ascii_digit()
}

pub fn find_first_number(chars: &Vec<char>) -> Option<i64> {
    match chars.clone().into_iter().find(is_number)?.to_digit(10) {
        None => None,
        Some(t) => Some(t as i64),
    }
}

pub fn find_last_number(chars: &Vec<char>) -> Option<i64> {
    match chars
        .clone()
        .into_iter()
        .rev()
        .find(is_number)?
        .to_digit(10)
    {
        None => None,
        Some(t) => Some(t as i64),
    }
}

pub fn find_number(data: &str) -> Option<i64> {
    let chars: Vec<char> = data.chars().collect();
    if chars.is_empty() {
        return None;
    }

    Some(find_first_number(&chars)? * 10 + find_last_number(&chars)?)
}
