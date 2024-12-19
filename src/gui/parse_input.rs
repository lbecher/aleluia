pub fn parse_input(prefix: &str, value: &mut f32, input: &mut String) {
    if input.starts_with(prefix) {
        if let Some(num_str) = input.strip_prefix(prefix) {
            if let Ok(parsed_value) = num_str.trim().parse::<f32>() {
                *value = parsed_value;
                *input = format!("{prefix} {parsed_value}");
                return;
            }
        }
    } else {
        if let Ok(parsed_value) = input.trim().parse::<f32>() {
            *value = parsed_value;
            *input = format!("{prefix} {parsed_value}");
            return;
        }
    }
    *input = format!("Inválido!");
}