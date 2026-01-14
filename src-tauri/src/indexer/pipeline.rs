use crate::parsers::{parse_file, ParsedDocument};
use std::path::Path;

/// 인덱싱 파이프라인
/// 파일 → 파싱 → 청크 분할 → FTS 인덱싱 → (Phase 3) 임베딩 → 벡터 인덱싱
pub fn index_file(path: &Path) -> Result<IndexResult, IndexError> {
    // 1. 파일 파싱
    let document = parse_file(path).map_err(|e| IndexError::ParseError(e.to_string()))?;

    // 2. FTS 인덱싱
    // TODO: DB에 저장

    // 3. 벡터 인덱싱 (Phase 3)
    // TODO: 임베딩 생성 → usearch에 저장

    Ok(IndexResult {
        file_path: path.to_string_lossy().to_string(),
        chunks_count: document.chunks.len(),
        total_chars: document.content.len(),
    })
}

#[derive(Debug)]
pub struct IndexResult {
    pub file_path: String,
    pub chunks_count: usize,
    pub total_chars: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum IndexError {
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Database error: {0}")]
    DbError(String),
}
