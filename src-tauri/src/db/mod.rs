use rusqlite::{Connection, Result};
use std::path::Path;

/// 데이터베이스 초기화
pub fn init_database(db_path: &Path) -> Result<()> {
    let conn = Connection::open(db_path)?;

    // 파일 메타데이터 테이블
    conn.execute(
        "CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY,
            path TEXT UNIQUE NOT NULL,
            name TEXT NOT NULL,
            file_type TEXT NOT NULL,
            size INTEGER,
            modified_at INTEGER,
            hash TEXT,
            indexed_at INTEGER
        )",
        [],
    )?;

    // 청크 메타데이터 테이블
    conn.execute(
        "CREATE TABLE IF NOT EXISTS chunks (
            id INTEGER PRIMARY KEY,
            file_id INTEGER REFERENCES files(id) ON DELETE CASCADE,
            chunk_index INTEGER,
            start_offset INTEGER,
            end_offset INTEGER,
            page_number INTEGER,
            paragraph_number INTEGER
        )",
        [],
    )?;

    // FTS5 전문 검색 인덱스
    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS chunks_fts USING fts5(
            content,
            content_rowid='id',
            tokenize='unicode61'
        )",
        [],
    )?;

    // 감시 폴더 테이블
    conn.execute(
        "CREATE TABLE IF NOT EXISTS watched_folders (
            id INTEGER PRIMARY KEY,
            path TEXT UNIQUE NOT NULL,
            added_at INTEGER
        )",
        [],
    )?;

    // 인덱스 생성
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_files_path ON files(path)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_chunks_file_id ON chunks(file_id)",
        [],
    )?;

    tracing::info!("Database initialized at {:?}", db_path);
    Ok(())
}
