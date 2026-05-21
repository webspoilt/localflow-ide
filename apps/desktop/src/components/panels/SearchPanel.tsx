import { useState } from 'react';

export function SearchPanel() {
  const [query, setQuery] = useState('');
  const [results] = useState<{ path: string; line: number; text: string }[]>([]);

  return (
    <div style={{ padding: '8px 0' }}>
      <div style={{
        display: 'flex',
        alignItems: 'center',
        gap: 6,
        margin: '0 12px 8px',
        padding: '5px 8px',
        background: 'var(--bg-tertiary)',
        borderRadius: 'var(--radius-sm)',
        border: '1px solid var(--border-color)',
      }}>
        <svg width="13" height="13" viewBox="0 0 16 16" fill="none" style={{ color: 'var(--text-muted)', flexShrink: 0 }}>
          <circle cx="6.5" cy="6.5" r="4.5" stroke="currentColor" strokeWidth="1.5" />
          <path d="M10 10L13 13" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" />
        </svg>
        <input
          type="text"
          value={query}
          onChange={(e) => { setQuery(e.target.value); }}
          placeholder="Search files..."
          autoFocus
          style={{
            flex: 1,
            background: 'transparent',
            border: 'none',
            color: 'var(--text-primary)',
            outline: 'none',
            fontSize: 12,
            fontFamily: 'var(--font-sans)',
          }}
        />
        <span style={{ fontSize: 10, color: 'var(--text-muted)' }}>ESC</span>
      </div>

      {results.length > 0 ? (
        <div style={{ padding: '0 8px' }}>
          {results.map((r, i) => (
            <div key={i} style={{ padding: '4px 8px', borderRadius: 'var(--radius-sm)', cursor: 'pointer' }}
              onMouseEnter={(e) => e.currentTarget.style.background = 'var(--bg-hover)'}
              onMouseLeave={(e) => e.currentTarget.style.background = 'transparent'}
            >
              <div style={{ fontSize: 11, color: 'var(--text-muted)', fontFamily: 'var(--font-mono)' }}>{r.path}:{r.line}</div>
              <div style={{ fontSize: 12, color: 'var(--text-secondary)', marginTop: 2 }}>{r.text}</div>
            </div>
          ))}
        </div>
      ) : (
        <div style={{ padding: '24px 12px', textAlign: 'center', color: 'var(--text-muted)', fontSize: 11 }}>
          {query ? 'No results' : 'Search across all files in your workspace'}
        </div>
      )}
    </div>
  );
}