pub fn format(key: &str, value: &str) -> String {
    let escaped = value.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('$', "\\$");
    format!("set -gx {} \"{}\"", key, escaped)
}
