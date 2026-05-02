pub fn format(key: &str, value: &str) -> String {
    format!("set \"{}={}\"", key, value)
}
