use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexStatus {
    pub total_files: usize,
    pub indexed_files: usize,
    pub pending_files: usize,
    pub is_indexing: bool,
    pub watched_folders: Vec<String>,
}

/// 감시 폴더 추가
#[tauri::command]
pub async fn add_folder(path: String) -> Result<(), String> {
    tracing::info!("Adding folder to watch: {}", path);
    // TODO: Implement folder watching
    Ok(())
}

/// 감시 폴더 제거
#[tauri::command]
pub async fn remove_folder(path: String) -> Result<(), String> {
    tracing::info!("Removing folder from watch: {}", path);
    // TODO: Implement folder removal
    Ok(())
}

/// 인덱스 상태 조회
#[tauri::command]
pub async fn get_index_status() -> Result<IndexStatus, String> {
    Ok(IndexStatus {
        total_files: 0,
        indexed_files: 0,
        pending_files: 0,
        is_indexing: false,
        watched_folders: vec![],
    })
}
