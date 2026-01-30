pub fn sanitize_name(name: &str) -> String {
    let sanitized: String = name
        .chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | ' ' | '.' => c,
            _ => '_',
        })
        .collect();

    let collapsed = sanitized.split_whitespace().collect::<Vec<_>>().join(" ");

    if collapsed.is_empty() {
        "unnamed_item".to_string()
    } else {
        collapsed.chars().take(200).collect()
    }
}
