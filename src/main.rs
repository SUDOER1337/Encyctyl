use encyctyl_core::VaultConfig;
use encyctyl_storage::Db;
use encyctyl_vault::Vault;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());

    let vault = Vault::new(VaultConfig::new(root.into()));
    let db = Db::open_in_memory()?;

    let all_notes = vault.discover_notes()?;
    let notes: Vec<_> = all_notes
        .iter()
        .filter(|p| !p.to_string_lossy().contains("Example Projects"))
        .collect();

    if notes.is_empty() {
        println!("No .md files found under {}", vault.root().display());
        return Ok(());
    }

    println!("Found {} note(s):\n", notes.len());

    for note in &notes {
        let rel = note.strip_prefix(vault.root()).unwrap_or(note);
        match vault.read_and_index_note(rel, &db) {
            Ok(parsed) => {
                let title = parsed
                    .frontmatter
                    .as_ref()
                    .and_then(|f| f.title.as_deref())
                    .unwrap_or("(no title)");
                let tags = if parsed.tags.is_empty() {
                    String::new()
                } else {
                    format!(" [{}]", parsed.tags.join(", "))
                };
                let links = if parsed.wiki_links.is_empty() {
                    String::new()
                } else {
                    format!(
                        " → {}",
                        parsed
                            .wiki_links
                            .iter()
                            .map(|l| l.target.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                };
                println!("  {}  {}{}{}", rel.display(), title, tags, links);
            }
            Err(e) => {
                println!("  {} — error: {}", rel.display(), e);
            }
        }
    }

    let count = db.all_notes()?.len();
    println!("\nIndexed {} notes in SQLite.", count);

    Ok(())
}
