use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub file_path: String,
    pub file_name: String,
    pub chunk_index: u32,
    pub content_preview: String,
    pub score: f32,
    pub highlight_ranges: Vec<(usize, usize)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub total_count: usize,
    pub search_time_ms: u64,
}

/// 키워드 검색 (FTS5)
#[tauri::command]
pub async fn search_keyword(query: String) -> Result<SearchResponse, String> {
    // TODO: Implement FTS5 search
    tracing::info!("Keyword search: {}", query);

    Ok(SearchResponse {
        results: vec![],
        total_count: 0,
        search_time_ms: 0,
    })
}

/// 시맨틱 검색 (벡터)
#[tauri::command]
pub async fn search_semantic(query: String) -> Result<SearchResponse, String> {
    // TODO: Implement vector search
    tracing::info!("Semantic search: {}", query);

    Ok(SearchResponse {
        results: vec![],
        total_count: 0,
        search_time_ms: 0,
    })
}
