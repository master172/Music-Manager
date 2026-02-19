use regex::Regex;

pub fn tokenize(input: &str) -> Vec<String> {
    let re = Regex::new(r#""([^"]+)"|(\S+)"#).unwrap();

    let mut tokens = vec![];

    for cap in re.captures_iter(input) {
        if let Some(quoted) = cap.get(1) {
            tokens.push(quoted.as_str().to_string())
        } else if let Some(word) = cap.get(2) {
            tokens.push(word.as_str().to_string());
        }
    }

    tokens
}
