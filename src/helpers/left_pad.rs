use super::get_spaces::get_spaces;

#[allow(dead_code)]
pub fn left_pad(content: &str, total_width: u8) -> String {
    return format!(
        "{}{}",
        get_spaces(total_width - content.chars().count() as u8),
        content
    );
}
