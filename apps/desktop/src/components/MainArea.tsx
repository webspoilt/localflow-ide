import { useCallback } from 'react';
import { useWorkspaceStore } from '@zynta/state';
import { EditorTabs } from './EditorTabs';
import { FileText } from 'lucide-react';

export function MainArea() {
  const activeTabId = useWorkspaceStore((s) => s.activeTabId);
  const openTabs = useWorkspaceStore((s) => s.openTabs);
  const openFile = useWorkspaceStore((s) => s.openFile);

  return (
    <div className="main-area">
      {openTabs.length > 0 && <EditorTabs />}
      <div className="editor-content">
        {activeTabId ? (
          <FileViewer tabId={activeTabId} />
        ) : (
          <div className="editor-empty">
            <FileText size={48} className="editor-empty-icon" />
            <h2>Zynta Studio</h2>
            <p>
              Open a file from the explorer to start editing.<br />
              The terminal runs your tasks in real time.
            </p>
            <div className="editor-empty-hint">
              <kbd>Ctrl</kbd>+<kbd>K</kbd>
              <span>Command palette</span>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

function FileViewer({ tabId }: { tabId: string }) {
  const openTabs = useWorkspaceStore((s) => s.openTabs);
  const activeTab = openTabs.find((t) => t.id === tabId);

  if (!activeTab) {
    return (
      <div className="editor-empty">
        <p>File not found</p>
      </div>
    );
  }

  return (
    <div className="task-output" style={{ flex: 1 }}>
      <div className="task-output-header">
        <span style={{ color: 'var(--text-secondary)', fontSize: 12 }}>{activeTab.fileName}</span>
        <span style={{ color: 'var(--text-muted)', fontSize: 11 }}>
          {activeTab.filePath}
        </span>
        <div style={{ flex: 1 }} />
        <span className="task-status-badge" style={{ background: 'var(--bg-elevated)', color: 'var(--text-muted)' }}>
          {activeTab.language}
        </span>
      </div>

      <div style={{
        flex: 1,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        padding: '48px 24px',
        textAlign: 'center',
      }}>
        <div>
          <p style={{ color: 'var(--text-secondary)', marginBottom: 8, fontSize: 14, fontWeight: 500 }}>
            {activeTab.fileName}
          </p>
          <p style={{ color: 'var(--text-muted)', fontSize: 12 }}>
            {activeTab.language} file
          </p>
          <p style={{ color: 'var(--text-muted)', fontSize: 12, marginTop: 16, maxWidth: 320, margin: '16px auto 0' }}>
            Editor integration coming soon. Content is rendered through Tauri's IPC bridge.
          </p>
        </div>
      </div>
    </div>
  );
}