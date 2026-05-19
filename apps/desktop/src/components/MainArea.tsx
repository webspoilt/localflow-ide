import { useWorkspaceStore } from '@zynta/state';
import { EditorTabs } from './EditorTabs';

export function MainArea() {
  const activeTabId = useWorkspaceStore((s) => s.activeTabId);
  const openTabs = useWorkspaceStore((s) => s.openTabs);

  return (
    <div className="main-area">
      {openTabs.length > 0 && <EditorTabs />}
      <div className="editor-content">
        {activeTabId ? (
          <FileViewer />
        ) : (
          <div className="editor-empty-state">
            <h2>Zynta Studio</h2>
            <p>
              Open a file from the explorer to get started.<br />
              Use Ctrl+` to toggle the terminal.
            </p>
          </div>
        )}
      </div>
    </div>
  );
}

function FileViewer() {
  const activeTabId = useWorkspaceStore((s) => s.activeTabId);
  const openTabs = useWorkspaceStore((s) => s.openTabs);
  const activeTab = openTabs.find((t) => t.id === activeTabId);

  if (!activeTab) {
    return (
      <div className="editor-empty-state">
        <p>File not found</p>
      </div>
    );
  }

  return (
    <div className="task-output" key={activeTab.id}>
      <dl className="metadata">
        <dt>File</dt>
        <dd>{activeTab.fileName}</dd>
        <dt>Path</dt>
        <dd>{activeTab.filePath}</dd>
        <dt>Language</dt>
        <dd>{activeTab.language}</dd>
      </dl>
      <p style={{ color: 'var(--text-muted)', textAlign: 'center', marginTop: 48 }}>
        Editor rendering not yet connected to runtime.
      </p>
    </div>
  );
}
