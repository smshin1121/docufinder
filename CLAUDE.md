# DocuFinder 프로젝트

로컬 문서 검색 앱 - HWPX, Office, PDF 지원 (Tauri + React)

## Quick Start

```bash
# 프론트엔드 개발 서버
pnpm dev

# Tauri 개발 모드 (Rust 빌드 포함)
pnpm tauri:dev

# 프로덕션 빌드
pnpm tauri:build
```

## 프로젝트 구조

```
Docufinder/
├── src-tauri/              # Rust 백엔드
│   ├── src/
│   │   ├── main.rs         # 앱 진입점
│   │   ├── lib.rs          # Tauri 설정
│   │   ├── commands/       # IPC 커맨드
│   │   ├── parsers/        # 파일 파서 (hwpx, docx, xlsx, pdf, txt)
│   │   ├── search/         # FTS5 + 벡터 검색
│   │   ├── indexer/        # 백그라운드 인덱싱
│   │   └── db/             # SQLite 스키마
│   └── Cargo.toml
├── src/                    # React 프론트엔드
│   ├── components/         # UI 컴포넌트
│   ├── hooks/              # 커스텀 훅
│   ├── store/              # 상태 관리
│   └── App.tsx
└── models/                 # 임베딩 모델 (Phase 3)
```

## 기술 스택

| 영역 | 기술 |
|------|------|
| Frontend | React 19 + TypeScript + Tailwind CSS |
| Backend | Rust + Tauri 2 |
| 검색 | SQLite FTS5 (키워드) + usearch (벡터) |
| 파싱 | zip, quick-xml, calamine |
| 파일 감시 | notify |

## 주요 명령어

| 명령어 | 설명 |
|--------|------|
| `pnpm dev` | Vite 개발 서버 |
| `pnpm tauri:dev` | Tauri 개발 모드 |
| `pnpm tauri:build` | MSI 설치파일 생성 |

## 개발 Phase

| Phase | 내용 | 상태 |
|-------|------|------|
| 1 | 기반 구축 (Tauri 셋업, TXT 검색) | 🔄 진행중 |
| 2 | 파일 파서 (HWPX, DOCX, XLSX, PDF) | ⏳ |
| 3 | 시맨틱 검색 (ONNX, e5-small) | ⏳ |
| 4 | 고급 기능 (실시간 감시, 미리보기) | ⏳ |
| 5 | 배포 (MSI, 자동 업데이트) | ⏳ |

## 참고 문서

| 문서 | 설명 |
|------|------|
| [PLAN.md](PLAN.md) | 상세 설계 계획 |
| [Tauri 문서](https://tauri.app) | 공식 문서 |

## 코딩 컨벤션

- TypeScript strict mode
- Rust 2021 edition
- 함수형 React 컴포넌트
- Tailwind CSS 유틸리티

## 연관 프로젝트

- **Auto_maeri**: HWPX 파서 로직 참고 (`packages/core/src/parser/`)
