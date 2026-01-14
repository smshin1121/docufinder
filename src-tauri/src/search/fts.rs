use rusqlite::Connection;

/// FTS5 키워드 검색
pub fn search(conn: &Connection, query: &str, limit: usize) -> Result<Vec<FtsResult>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT rowid, content, rank
         FROM chunks_fts
         WHERE chunks_fts MATCH ?
         ORDER BY rank
         LIMIT ?"
    )?;

    let results = stmt.query_map([query, &limit.to_string()], |row| {
        Ok(FtsResult {
            chunk_id: row.get(0)?,
            content: row.get(1)?,
            score: row.get(2)?,
        })
    })?;

    results.collect()
}

#[derive(Debug)]
pub struct FtsResult {
    pub chunk_id: i64,
    pub content: String,
    pub score: f64,
}
