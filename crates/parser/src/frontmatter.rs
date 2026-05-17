use super::Frontmatter;
use std::collections::HashMap;

pub fn extract_frontmatter(content: &str) -> (Option<Frontmatter>, Option<&str>) {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return (None, None);
    }

    let after_opening = &trimmed[3..];
    let close_pos = match after_opening.find("\n---") {
        Some(p) => p,
        None => return (None, None),
    };

    // close_pos is the position of '\n' in after_opening that precedes the closing `---`
    // after_opening[close_pos..close_pos+4] = "\n---"
    let yaml_end = close_pos; // yaml content is after_opening[0..close_pos]
    let fm_end = close_pos + 4; // after_opening[fm_end] is first char after closing ---

    let yaml_str = after_opening[..yaml_end].trim();
    let raw_map: HashMap<String, String> = serde_yaml::from_str(yaml_str).unwrap_or_default();

    let body = after_opening[fm_end..].trim_start();
    let body = if body.is_empty() { None } else { Some(body) };

    let frontmatter = Frontmatter {
        title: raw_map.get("title").cloned(),
        tags: raw_map
            .get("tags")
            .map(|t| {
                serde_yaml::from_str::<Vec<String>>(t)
                    .unwrap_or_else(|_| t.split(',').map(|s| s.trim().to_string()).collect())
            })
            .unwrap_or_default(),
        date: raw_map.get("date").cloned(),
        raw: raw_map,
    };

    (Some(frontmatter), body)
}
