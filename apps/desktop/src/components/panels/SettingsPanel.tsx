import { useState } from 'react';

const sections = [
  { id: 'appearance', label: 'Appearance' },
  { id: 'editor', label: 'Editor' },
  { id: 'terminal', label: 'Terminal' },
  { id: 'runtime', label: 'Runtime' },
  { id: 'security', label: 'Security' },
];

export function SettingsPanel() {
  const [activeSection, setActiveSection] = useState('appearance');

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
      <div style={{
        display: 'flex',
        gap: 1,
        padding: '8px 12px',
        borderBottom: '1px solid var(--border-color)',
        overflowX: 'auto',
      }}>
        {sections.map((s) => (
          <button
            key={s.id}
            onClick={() => { setActiveSection(s.id); }}
            style={{
              padding: '3px 10px',
              fontSize: 11,
              fontWeight: 500,
              color: activeSection === s.id ? 'var(--text-primary)' : 'var(--text-muted)',
              background: activeSection === s.id ? 'var(--bg-elevated)' : 'transparent',
              border: 'none',
              borderRadius: 'var(--radius-sm)',
              cursor: 'pointer',
              whiteSpace: 'nowrap',
              transition: 'all var(--transition-fast)',
            }}
          >
            {s.label}
          </button>
        ))}
      </div>

      <div style={{ flex: 1, overflow: 'auto', padding: '16px 12px' }}>
        {activeSection === 'appearance' && (
          <div style={{ display: 'flex', flexDirection: 'column', gap: 12 }}>
            <SettingRow label="Theme" description="Color theme for the interface">
              <select style={{
                background: 'var(--bg-elevated)',
                color: 'var(--text-primary)',
                border: '1px solid var(--border-color)',
                borderRadius: 'var(--radius-sm)',
                padding: '4px 8px',
                fontSize: 12,
              }}>
                <option>Dark (default)</option>
                <option>Light</option>
                <option>System</option>
              </select>
            </SettingRow>
            <SettingRow label="Font size" description="UI font size">
              <select style={{
                background: 'var(--bg-elevated)',
                color: 'var(--text-primary)',
                border: '1px solid var(--border-color)',
                borderRadius: 'var(--radius-sm)',
                padding: '4px 8px',
                fontSize: 12,
              }}>
                <option>12px</option>
                <option>13px</option>
                <option>14px</option>
              </select>
            </SettingRow>
          </div>
        )}

        {activeSection !== 'appearance' && (
          <div style={{ color: 'var(--text-muted)', fontSize: 11 }}>
            {sections.find((s) => s.id === activeSection)?.label} settings coming soon.
          </div>
        )}
      </div>
    </div>
  );
}

function SettingRow({ label, description, children }: { label: string; description: string; children: React.ReactNode }) {
  return (
    <div style={{
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'space-between',
      padding: '8px 12px',
      background: 'var(--bg-elevated)',
      borderRadius: 'var(--radius-sm)',
      border: '1px solid var(--border-color)',
    }}>
      <div>
        <div style={{ fontSize: 12, fontWeight: 500, color: 'var(--text-primary)' }}>{label}</div>
        <div style={{ fontSize: 11, color: 'var(--text-muted)', marginTop: 1 }}>{description}</div>
      </div>
      {children}
    </div>
  );
}