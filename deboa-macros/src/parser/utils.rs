use regex::Regex;

pub fn extract_path_params(path: &str) -> String {
    Regex::new(r"<(\w*):\&{0,1}\w*>")
        .expect("Invalid path")
        .replace_all(path, "{$1}")
        .to_string()
}
