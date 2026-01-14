use super::{DocumentChunk, DocumentMetadata, ParseError, ParsedDocument};
use std::fs;
use std::path::Path;

pub fn parse(path: &Path) -> Result<ParsedDocument, ParseError> {
    let content = fs::read_to_string(path)?;

    // 청크 분할 (512자, 64자 오버랩)
    let chunks = chunk_text(&content, 512, 64);

    Ok(ParsedDocument {
        content: content.clone(),
        metadata: DocumentMetadata {
            title: path.file_stem().and_then(|s| s.to_str()).map(String::from),
            author: None,
            created_at: None,
            page_count: None,
        },
        chunks,
    })
}

fn chunk_text(text: &str, chunk_size: usize, overlap: usize) -> Vec<DocumentChunk> {
    let mut chunks = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let total_len = chars.len();

    if total_len == 0 {
        return chunks;
    }

    let step = chunk_size.saturating_sub(overlap).max(1);
    let mut start = 0;

    while start < total_len {
        let end = (start + chunk_size).min(total_len);
        let chunk_content: String = chars[start..end].iter().collect();

        chunks.push(DocumentChunk {
            content: chunk_content,
            start_offset: start,
            end_offset: end,
            page_number: None,
        });

        start += step;

        // 마지막 청크면 종료
        if end >= total_len {
            break;
        }
    }

    chunks
}
