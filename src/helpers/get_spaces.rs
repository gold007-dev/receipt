pub fn get_spaces(amount: u8) -> String {
    let mut spaces = String::new();
    for _ in 0..amount {
        spaces.push(' ');
    }
    spaces
}