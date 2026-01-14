/// 벡터 검색 (Phase 3에서 구현)
/// usearch + ONNX 임베딩 사용 예정

#[derive(Debug)]
pub struct VectorResult {
    pub chunk_id: i64,
    pub score: f32,
}

/// 벡터 검색 수행
pub fn search(_query_embedding: &[f32], _limit: usize) -> Vec<VectorResult> {
    // Phase 3에서 구현 예정
    // 1. 쿼리 텍스트 → 임베딩 변환
    // 2. usearch로 nearest neighbor 검색
    vec![]
}
