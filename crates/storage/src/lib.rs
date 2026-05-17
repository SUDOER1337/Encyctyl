use encyctyl_core::Error;
use rusqlite::{params, Connection};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

fn db_err(e: rusqlite::Error) -> Error {
    Error::Database(e.to_string())
}

pub struct Db {
    conn: Mutex<Connection>,
}

#[derive(Debug, Clone)]
pub struct NoteMeta {
    pub id: i64,
    pub path: PathBuf,
    pub title: Option<String>,
    pub content_hash: Option<String>,
    pub tags: Vec<String>,
    pub links: Vec<String>,
    pub backlinks: Vec<(String, String)>,
    pub created_at: String,
    pub updated_at: String,
}

impl Db {
    pub fn open(path: &Path) -> Result<Self, Error> {
        let conn = Connection::open(path).map_err(db_err)?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.migrate()?;
        Ok(db)
    }

    pub fn open_in_memory() -> Result<Self, Error> {
        let conn = Connection::open_in_memory().map_err(db_err)?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.migrate()?;
        Ok(db)
    }

    fn migrate(&self) -> Result<(), Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS notes (
                id      INTEGER PRIMARY KEY AUTOINCREMENT,
                path    TEXT UNIQUE NOT NULL,
                title   TEXT,
                content_hash TEXT,
                created_at TEXT DEFAULT (datetime('now')),
                updated_at TEXT DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS tags (
                id      INTEGER PRIMARY KEY AUTOINCREMENT,
                note_id INTEGER NOT NULL REFERENCES notes(id) ON DELETE CASCADE,
                tag     TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_tags_tag ON tags(tag);

            CREATE TABLE IF NOT EXISTS links (
                id        INTEGER PRIMARY KEY AUTOINCREMENT,
                source_id INTEGER NOT NULL REFERENCES notes(id) ON DELETE CASCADE,
                target    TEXT NOT NULL,
                alias     TEXT
            );

            CREATE INDEX IF NOT EXISTS idx_links_target ON links(target);
            ",
        )
        .map_err(db_err)?;
        Ok(())
    }

    pub fn upsert_note(
        &self,
        path: &Path,
        title: Option<&str>,
        content_hash: Option<&str>,
        tags: &[String],
        links: &[(&str, Option<&str>)],
    ) -> Result<i64, Error> {
        let conn = self.conn.lock().unwrap();
        let path_str = path.to_string_lossy();

        conn.execute(
            "INSERT INTO notes (path, title, content_hash)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(path) DO UPDATE SET
               title = COALESCE(?2, title),
               content_hash = COALESCE(?3, content_hash),
               updated_at = datetime('now')",
            params![path_str.as_ref(), title, content_hash],
        )
        .map_err(db_err)?;

        let note_id: i64 = conn
            .query_row(
                "SELECT id FROM notes WHERE path = ?1",
                params![path_str.as_ref()],
                |row| row.get(0),
            )
            .map_err(db_err)?;

        conn.execute("DELETE FROM tags WHERE note_id = ?1", params![note_id])
            .map_err(db_err)?;
        for tag in tags {
            conn.execute(
                "INSERT INTO tags (note_id, tag) VALUES (?1, ?2)",
                params![note_id, tag],
            )
            .map_err(db_err)?;
        }

        conn.execute("DELETE FROM links WHERE note_id = ?1", params![note_id])
            .map_err(db_err)?;
        for (target, alias) in links {
            conn.execute(
                "INSERT INTO links (source_id, target, alias) VALUES (?1, ?2, ?3)",
                params![note_id, target, alias],
            )
            .map_err(db_err)?;
        }

        Ok(note_id)
    }

    pub fn get_note_by_path(&self, path: &Path) -> Result<Option<NoteMeta>, Error> {
        let conn = self.conn.lock().unwrap();
        let path_str = path.to_string_lossy();

        let note = conn
            .query_row(
                "SELECT id, path, title, content_hash, created_at, updated_at
                 FROM notes WHERE path = ?1",
                params![path_str.as_ref()],
                |row| {
                    Ok(NoteMeta {
                        id: row.get(0)?,
                        path: PathBuf::from(row.get::<_, String>(1)?),
                        title: row.get(2)?,
                        content_hash: row.get(3)?,
                        tags: Vec::new(),
                        links: Vec::new(),
                        backlinks: Vec::new(),
                        created_at: row.get(4)?,
                        updated_at: row.get(5)?,
                    })
                },
            )
            .ok();

        let note = match note {
            Some(n) => n,
            None => return Ok(None),
        };

        let id = note.id;
        let note = NoteMeta {
            tags: self.get_tags_for_note(&conn, id),
            links: self.get_links_for_note(&conn, id),
            backlinks: self.get_backlinks_for_note(&conn, &path_str),
            ..note
        };

        Ok(Some(note))
    }

    fn get_tags_for_note(&self, conn: &Connection, note_id: i64) -> Vec<String> {
        let mut stmt = conn
            .prepare("SELECT tag FROM tags WHERE note_id = ?1 ORDER BY tag")
            .unwrap();
        stmt.query_map(params![note_id], |row| row.get::<_, String>(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect()
    }

    fn get_links_for_note(&self, conn: &Connection, note_id: i64) -> Vec<String> {
        let mut stmt = conn
            .prepare("SELECT target FROM links WHERE source_id = ?1")
            .unwrap();
        stmt.query_map(params![note_id], |row| row.get::<_, String>(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect()
    }

    fn get_backlinks_for_note(&self, conn: &Connection, path: &str) -> Vec<(String, String)> {
        let mut stmt = conn
            .prepare(
                "SELECT n.path, COALESCE(l.alias, '')
                 FROM links l
                 JOIN notes n ON n.id = l.source_id
                 WHERE l.target = ?1",
            )
            .unwrap();
        stmt.query_map(params![path], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect()
    }

    pub fn search_by_tag(&self, tag: &str) -> Result<Vec<NoteMeta>, Error> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT n.id, n.path, n.title, n.content_hash, n.created_at, n.updated_at
                 FROM notes n
                 JOIN tags t ON t.note_id = n.id
                 WHERE t.tag = ?1
                 ORDER BY n.updated_at DESC",
            )
            .map_err(db_err)?;

        let rows = stmt
            .query_map(params![tag], |row| {
                let id: i64 = row.get(0)?;
                Ok(NoteMeta {
                    id,
                    path: PathBuf::from(row.get::<_, String>(1)?),
                    title: row.get(2)?,
                    content_hash: row.get(3)?,
                    tags: Vec::new(),
                    links: Vec::new(),
                    backlinks: Vec::new(),
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            })
            .map_err(db_err)?
            .filter_map(|r| r.ok())
            .collect::<Vec<_>>();

        Ok(rows)
    }

    pub fn all_notes(&self) -> Result<Vec<NoteMeta>, Error> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare(
                "SELECT id, path, title, content_hash, created_at, updated_at
                 FROM notes ORDER BY updated_at DESC",
            )
            .map_err(db_err)?;

        let rows = stmt
            .query_map([], |row| {
                let id: i64 = row.get(0)?;
                Ok(NoteMeta {
                    id,
                    path: PathBuf::from(row.get::<_, String>(1)?),
                    title: row.get(2)?,
                    content_hash: row.get(3)?,
                    tags: Vec::new(),
                    links: Vec::new(),
                    backlinks: Vec::new(),
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            })
            .map_err(db_err)?
            .filter_map(|r| r.ok())
            .collect();

        Ok(rows)
    }
}
