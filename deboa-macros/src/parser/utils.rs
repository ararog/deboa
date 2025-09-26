use regex::Regex;

pub fn extract_path_params(path: &str) -> String {
    Regex::new(r"<(\w*):\&{0,1}\w*>")
        .expect("Invalid path")
        .replace_all(path, "{$1}")
        .to_string()
}

pub fn extract_params_from_path(path: &str) -> Vec<(String, String)> {
    let re = Regex::new(r"<(\w*):(\&{0,1}\w*)>")
        .expect("Invalid path");
    let mut params = Vec::new();
    for cap in re.captures_iter(path) {
        params.push((cap[1].to_string(), cap[2].to_string()));
    }
    params
}
