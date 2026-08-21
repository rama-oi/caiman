/// Truncate a string to fit within `max_width` display columns,
/// appending an ellipsis ("…") when it doesn't fit.
pub fn truncate_label(label: &str, max_width: u16) -> String {
    let max_width = max_width as usize;

    if max_width == 0 {
        return String::new();
    }

    if label.chars().count() <= max_width {
        return label.to_string();
    }

    if max_width == 1 {
        return "…".to_string();
    }

    let truncated: String = label.chars().take(max_width - 1).collect();
    format!("{truncated}…")
}

pub fn wrap_help_items(items: &[&str], width: u16) -> Vec<String> {
    let width = width as usize;
    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();

    for item in items {
        let extra = if current.is_empty() { 0 } else { 2 };

        if !current.is_empty() && current.len() + extra + item.len() > width {
            lines.push(std::mem::take(&mut current));
        }

        if !current.is_empty() {
            current.push_str("  ");
        }
        current.push_str(item);
    }

    if !current.is_empty() || lines.is_empty() {
        lines.push(current);
    }

    lines
}
