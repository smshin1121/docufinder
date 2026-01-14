use super::{DocumentChunk, DocumentMetadata, ParseError, ParsedDocument};
use std::path::Path;

/// XLSX 파일 파싱
/// TODO: calamine으로 구현
pub fn parse(path: &Path) -> Result<ParsedDocument, ParseError> {
    // Phase 2에서 구현 예정

    tracing::warn!("XLSX parser not yet implemented: {:?}", path);

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
