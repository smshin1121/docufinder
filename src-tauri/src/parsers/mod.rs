pub mod docx;
pub mod hwpx;
pub mod pdf;
pub mod txt;
pub mod xlsx;

use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unsupported file type: {0}")]
    UnsupportedFileType(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    ParseError(String),
}

/// 파싱 결과
#[derive(Debug)]
pub struct ParsedDocument {
    pub content: String,
    pub metadata: DocumentMetadata,
    pub chunks: Vec<DocumentChunk>,
}

#[derive(Debug)]
pub struct DocumentMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub created_at: Option<i64>,
    pub page_count: Option<usize>,
}

#[derive(Debug)]
pub struct DocumentChunk {
    pub content: String,
    pub start_offset: usize,
    pub end_offset: usize,
    pub page_number: Option<usize>,
}

/// 파일 확장자로 파서 선택 후 파싱
pub fn parse_file(path: &Path) -> Result<ParsedDocument, ParseError> {
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "txt" | "md" => txt::parse(path),
        "hwpx" => hwpx::parse(path),
        "docx" => docx::parse(path),
        "xlsx" => xlsx::parse(path),
        "pdf" => pdf::parse(path),
        _ => Err(ParseError::UnsupportedFileType(extension)),
    }
}
