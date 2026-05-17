mod frontmatter;
mod tags;
mod wikilinks;

use encyctyl_core::Error;
use pulldown_cmark::{Event, Parser};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TextRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub tags: Vec<String>,
    pub date: Option<String>,
    pub raw: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct WikiLink {
    pub target: String,
    pub alias: Option<String>,
    pub position: TextRange,
}

#[derive(Debug, Clone)]
pub struct ParsedNote {
    pub frontmatter: Option<Frontmatter>,
    pub events: Vec<Event<'static>>,
    pub wiki_links: Vec<WikiLink>,
    pub tags: Vec<String>,
}

pub fn parse_note(content: &str) -> Result<ParsedNote, Error> {
    let (frontmatter, body) = frontmatter::extract_frontmatter(content);
    let body = body.unwrap_or(content);

    let mut events = Vec::new();
    let parser = Parser::new(body);

    let mut wiki_links = Vec::new();
    let mut tags_vec = Vec::new();

    for event in parser {
        match &event {
            Event::Text(text) => {
                let text_str = text.to_string();
                wiki_links.extend(wikilinks::extract_wikilinks(&text_str));
                tags_vec.extend(tags::extract_tags(&text_str));
            }
            _ => {}
        }
        events.push(event.into_static());
    }

    tags_vec.sort();
    tags_vec.dedup();

    Ok(ParsedNote {
        frontmatter,
        events,
        wiki_links,
        tags: tags_vec,
    })
}
