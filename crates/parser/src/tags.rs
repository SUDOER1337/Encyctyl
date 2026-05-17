/// Scans text for `#tag` patterns.
/// Tags must start with `#` followed by alphanumerics, hyphens, or underscores.
/// A `#` at the very start of a word qualifies; tags inside code spans are NOT filtered here
/// (that filtering should happen at the pulldown-cmark event level).
pub fn extract_tags(text: &str) -> Vec<String> {
    let mut tags = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'#' {
            let start = i + 1;
            let mut j = start;
            while j < bytes.len()
                && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'-' || bytes[j] == b'_')
            {
                j += 1;
            }
            if j > start {
                let tag = text[start..j].to_string();
                tags.push(tag);
                i = j;
                continue;
            }
        }
        i += 1;
    }

    tags
}
