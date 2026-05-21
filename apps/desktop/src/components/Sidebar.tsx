import { useCallback } from 'react';
import { useUIStore } from '@local-flow/state';
import { ExplorerPanel } from './panels/ExplorerPanel';
import { SearchPanel } from './panels/SearchPanel';
import { ExtensionsPanel } from './panels/ExtensionsPanel';
import { SettingsPanel } from './panels/SettingsPanel';
import { CognitivePanel } from './panels/CognitivePanel';
import { X } from 'lucide-react';

const panelLabels: Record<string, string> = {
  explorer: 'Explorer',
  search: 'Search',
  extensions: 'Extensions',
  settings: 'Settings',
  cognitive: 'Cognitive Dashboard',
};

export function Sidebar() {
  const activePanel = useUIStore((s) => s.activePanel);
  const toggleSidebar = useUIStore((s) => s.toggleSidebar);

  const renderPanel = useCallback(() => {
    switch (activePanel) {
      case 'explorer':
        return <ExplorerPanel />;
      case 'search':
        return <SearchPanel />;
      case 'extensions':
        return <ExtensionsPanel />;
      case 'settings':
        return <SettingsPanel />;
      case 'cognitive':
        return <CognitivePanel />;
      default:
        return <ExplorerPanel />;
    }
  }, [activePanel]);

  return (
    <aside className="sidebar">
      <div className="sidebar-header">
        <span className="sidebar-title">{panelLabels[activePanel] ?? 'Explorer'}</span>
        <div className="sidebar-actions">
          <button
            className="terminal-btn"
            onClick={toggleSidebar}
            title="Close sidebar"
          >
            <X size={14} />
          </button>
        </div>
      </div>
      <div className="sidebar-content">
        {renderPanel()}
      </div>
    </aside>
  );
}