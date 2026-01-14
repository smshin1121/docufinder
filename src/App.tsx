import { useState } from "react";

function App() {
  const [query, setQuery] = useState("");

  return (
    <div className="min-h-screen bg-gray-900 text-white">
      {/* Header */}
      <header className="border-b border-gray-800 px-6 py-4">
        <h1 className="text-2xl font-bold">DocuFinder</h1>
        <p className="text-gray-400 text-sm">로컬 문서 검색 시스템</p>
      </header>

      {/* Search Bar */}
      <div className="p-6">
        <div className="max-w-2xl mx-auto">
          <input
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="검색어를 입력하세요..."
            className="w-full px-4 py-3 bg-gray-800 border border-gray-700 rounded-lg
                       focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
                       text-white placeholder-gray-500"
          />
        </div>
      </div>

      {/* Results Area */}
      <main className="px-6 pb-6">
        <div className="max-w-4xl mx-auto">
          <div className="text-center text-gray-500 py-12">
            <svg
              className="w-16 h-16 mx-auto mb-4 opacity-50"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={1.5}
                d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
              />
            </svg>
            <p>폴더를 선택하고 검색을 시작하세요</p>
          </div>
        </div>
      </main>

      {/* Status Bar */}
      <footer className="fixed bottom-0 left-0 right-0 bg-gray-800 border-t border-gray-700 px-4 py-2">
        <div className="flex justify-between text-sm text-gray-400">
          <span>인덱싱된 문서: 0개</span>
          <span>준비됨</span>
        </div>
      </footer>
    </div>
  );
}

export default App;
