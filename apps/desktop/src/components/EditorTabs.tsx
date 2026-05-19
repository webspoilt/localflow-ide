import { useWorkspaceStore } from '@zynta/state';
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
          onClick={() => setActiveTab(tab.id)}
        >
          {tab.isDirty && <span className="dirty-indicator" />}
          {tab.fileName}
          <button
            className="close-btn"
            onClick={(e) => {
              e.stopPropagation();
              closeTab(tab.id);
            }}
          >
            <X size={12} />
          </button>
        </div>
      ))}
    </div>
  );
}
