use super::{DocumentChunk, DocumentMetadata, ParseError, ParsedDocument};
use std::path::Path;

/// HWPX 파일 파싱
/// TODO: Auto_maeri의 HWPX 파서 로직을 Rust로 포팅
pub fn parse(path: &Path) -> Result<ParsedDocument, ParseError> {
    // Phase 2에서 구현 예정
    // Auto_maeri의 packages/core/src/parser/ 참고

    tracing::warn!("HWPX parser not yet implemented: {:?}", path);

    Ok(ParsedDocument {
        content: String::new(),
        metadata: DocumentMetadata {
            title: path.file_stem().and_then(|s| s.to_str()).map(String::from),
            author: None,
            created_at: None,
            page_count: None,
        },
        chunks: vec![],
    })
}
