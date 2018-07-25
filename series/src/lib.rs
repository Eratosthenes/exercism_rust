pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 || len > digits.len() { 
        return vec![String::from(""); digits.len() + 1];
    }
    digits.chars()
        .map(|x| format!("{}", x))
        .collect::<Vec<_>>()
        .windows(len)
        .map(|x| x.join(""))
        .collect()
}
