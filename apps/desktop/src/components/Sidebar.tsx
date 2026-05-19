import { useUIStore } from '@zynta/state';
import { ExplorerPanel } from './panels/ExplorerPanel';
import { SearchPanel } from './panels/SearchPanel';
import { ExtensionsPanel } from './panels/ExtensionsPanel';
import { SettingsPanel } from './panels/SettingsPanel';

export function Sidebar() {
  const activePanel = useUIStore((s) => s.activePanel);

  const renderPanel = () => {
    switch (activePanel) {
      case 'explorer':
        return <ExplorerPanel />;
      case 'search':
        return <SearchPanel />;
      case 'extensions':
        return <ExtensionsPanel />;
      case 'settings':
        return <SettingsPanel />;
      default:
        return <ExplorerPanel />;
    }
  };

  return (
    <div className="sidebar">
      <div className="sidebar-header">{activePanel.charAt(0).toUpperCase() + activePanel.slice(1)}</div>
      <div className="sidebar-content">{renderPanel()}</div>
    </div>
  );
}
