use super::{TextRange, WikiLink};

pub fn extract_wikilinks(text: &str) -> Vec<WikiLink> {
    let mut links = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0;

    while i < bytes.len().saturating_sub(1) {
        if bytes[i] == b'[' && bytes[i + 1] == b'[' {
            let start = i;
            i += 2;

            let mut depth = 2;
            let mut j = i;
            while j < bytes.len() {
                if bytes[j] == b']' && j + 1 < bytes.len() && bytes[j + 1] == b']' {
                    depth -= 1;
                    if depth == 0 {
                        let inner = &text[i..j];

                        let (target, alias) = inner.split_once('|').unwrap_or((inner, ""));
                        let target = target.trim().to_string();
                        let alias = if alias.is_empty() {
                            None
                        } else {
                            Some(alias.trim().to_string())
                        };

                        if !target.is_empty() {
                            links.push(WikiLink {
                                target,
                                alias,
                                position: TextRange { start, end: j + 2 },
                            });
                        }
                        i = j + 2;
                        break;
                    }
                    j += 2;
                } else {
                    j += 1;
                }
            }

            if j >= bytes.len() {
                break;
            }
        } else {
            i += 1;
        }
    }

    links
}
