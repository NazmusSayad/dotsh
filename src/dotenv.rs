pub fn parse(input: &str) -> Vec<(String, String)> {
    let mut result = Vec::new();
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((key, value)) = trimmed.split_once('=') else {
            continue;
        };
        result.push((key.trim().to_string(), value.trim().to_string()));
    }
    result
}
