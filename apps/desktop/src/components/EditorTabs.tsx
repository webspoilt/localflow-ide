import { useWorkspaceStore } from '@local-flow/state';
import { X } from 'lucide-react';

export function EditorTabs() {
  const openTabs = useWorkspaceStore((s) => s.openTabs);
  const activeTabId = useWorkspaceStore((s) => s.activeTabId);
  const setActiveTab = useWorkspaceStore((s) => s.setActiveTab);
  const closeTab = useWorkspaceStore((s) => s.closeTab);

  return (
    <div className="editor-tabs">
      {openTabs.map((tab) => (
        <div
          key={tab.id}
          className={`editor-tab ${tab.id === activeTabId ? 'active' : ''}`}
          onClick={() => { setActiveTab(tab.id); }}
          title={tab.filePath}
        >
          {tab.isDirty && <span className="dirty-dot" />}
          <span className="editor-tab-name">{tab.fileName}</span>
          <button
            className="close-btn"
            onClick={(e) => {
              e.stopPropagation();
              closeTab(tab.id);
            }}
            title={`Close ${tab.fileName}`}
          >
            <X size={12} />
          </button>
        </div>
      ))}
    </div>
  );
}