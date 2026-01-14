use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub search_mode: SearchMode,
    pub max_results: usize,
    pub chunk_size: usize,
    pub chunk_overlap: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SearchMode {
    Keyword,
    Semantic,
    Hybrid,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            search_mode: SearchMode::Hybrid,
            max_results: 50,
            chunk_size: 512,
            chunk_overlap: 64,
        }
    }
}

/// 설정 조회
#[tauri::command]
pub async fn get_settings() -> Result<Settings, String> {
    Ok(Settings::default())
}

/// 설정 업데이트
#[tauri::command]
pub async fn update_settings(settings: Settings) -> Result<(), String> {
    tracing::info!("Updating settings: {:?}", settings);
    // TODO: Persist settings
    Ok(())
}
