use super::get_spaces::get_spaces;

#[allow(dead_code)]
pub fn auto_pad(content: &str, total_width: u8) -> String {
    if (total_width + content.chars().count() as u8) % 2 == 0 {
        let pad = get_spaces((total_width - content.chars().count() as u8) / 2);
        return format!("{}{}{}", pad, content, pad);
    } else {
        return format!(
            "{}{}{}",
            get_spaces((total_width - content.chars().count() as u8) / 2 + 1),
            content,
            get_spaces((total_width - content.chars().count() as u8) / 2)
        );
    }
}