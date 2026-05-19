import { useUIStore, type PanelId } from '@zynta/state';
import { FileText, Search, Terminal, Puzzle, Settings, PanelRightClose, PanelRightOpen } from 'lucide-react';

const items: { id: PanelId; icon: typeof FileText; label: string }[] = [
  { id: 'explorer', icon: FileText, label: 'Explorer' },
  { id: 'search', icon: Search, label: 'Search' },
  { id: 'terminal', icon: Terminal, label: 'Terminal' },
  { id: 'extensions', icon: Puzzle, label: 'Extensions' },
  { id: 'settings', icon: Settings, label: 'Settings' },
];

export function ActivityBar() {
  const activePanel = useUIStore((s) => s.activePanel);
  const sidebarVisible = useUIStore((s) => s.sidebarVisible);
  const setActivePanel = useUIStore((s) => s.setActivePanel);
  const toggleSidebar = useUIStore((s) => s.toggleSidebar);

  return (
    <div className="activity-bar">
      {items.map((item) => (
        <button
          key={item.id}
          className={`activity-bar-btn ${activePanel === item.id && sidebarVisible ? 'active' : ''}`}
          onClick={() => {
            if (activePanel === item.id && sidebarVisible) {
              toggleSidebar();
            } else {
              setActivePanel(item.id);
              if (!sidebarVisible) toggleSidebar();
            }
          }}
          title={item.label}
        >
          <item.icon size={18} />
        </button>
      ))}
      <div className="activity-bar-spacer" />
      <button
        className="activity-bar-btn"
        onClick={toggleSidebar}
        title={sidebarVisible ? 'Close sidebar' : 'Open sidebar'}
      >
        {sidebarVisible ? <PanelRightClose size={16} /> : <PanelRightOpen size={16} />}
      </button>
    </div>
  );
}
