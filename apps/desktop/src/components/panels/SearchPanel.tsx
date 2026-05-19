import { useState } from 'react';
import { Search } from 'lucide-react';

export function SearchPanel() {
  const [query, setQuery] = useState('');
  const [results] = useState<string[]>([]);

  return (
    <div style={{ padding: 8 }}>
      <div style={{ display: 'flex', gap: 4, background: 'var(--bg-tertiary)', borderRadius: 'var(--radius-sm)', padding: '4px 8px', alignItems: 'center' }}>
        <Search size={14} style={{ color: 'var(--text-muted)', flexShrink: 0 }} />
        <input
          type="text"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Search files..."
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
      </div>
      {results.length === 0 && query && (
        <p style={{ color: 'var(--text-muted)', marginTop: 12, fontSize: 12, textAlign: 'center' }}>
          No results found
        </p>
      )}
      {results.length === 0 && !query && (
        <p style={{ color: 'var(--text-muted)', marginTop: 12, fontSize: 12, textAlign: 'center' }}>
          Search across files in your workspace
        </p>
      )}
    </div>
  );
}
