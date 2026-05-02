pub fn format(key: &str, value: &str) -> String {
    let escaped = value.replace('\'', "''");
    format!("$env:{} = '{}'", key, escaped)
}
