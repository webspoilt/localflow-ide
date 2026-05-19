export function ExtensionsPanel() {
  return (
    <div style={{ padding: '16px 12px', textAlign: 'center' }}>
      <div style={{
        width: 40,
        height: 40,
        borderRadius: 8,
        background: 'var(--bg-elevated)',
        border: '1px solid var(--border-color)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        margin: '0 auto 12px',
        color: 'var(--text-muted)',
      }}>
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
          <rect x="3" y="3" width="8" height="8" rx="1" />
          <rect x="13" y="3" width="8" height="8" rx="1" />
          <rect x="3" y="13" width="8" height="8" rx="1" />
          <rect x="13" y="13" width="8" height="8" rx="1" />
        </svg>
      </div>
      <p style={{ color: 'var(--text-secondary)', fontSize: 12, fontWeight: 500, marginBottom: 4 }}>
        Extensions
      </p>
      <p style={{ color: 'var(--text-muted)', fontSize: 11, lineHeight: 1.5 }}>
        Install extensions to add features to Zynta Studio.
      </p>
    </div>
  );
}