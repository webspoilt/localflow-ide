import { useCallback } from 'react';
import { useUIStore, type PanelId } from '@zynta/state';
import {
  FileText,
  Search,
  Terminal,
  Puzzle,
  Settings,
  GitBranch,
} from 'lucide-react';

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

  const handlePanelClick = useCallback(
    (id: PanelId) => {
      if (activePanel === id && sidebarVisible) {
        toggleSidebar();
      } else {
        setActivePanel(id);
        if (!sidebarVisible) toggleSidebar();
      }
    },
    [activePanel, sidebarVisible, setActivePanel, toggleSidebar],
  );

  return (
    <div className="activity-bar">
      {items.map((item) => (
        <button
          key={item.id}
          className={`activity-bar-btn ${activePanel === item.id && sidebarVisible ? 'active' : ''}`}
          onClick={() => handlePanelClick(item.id)}
          title={item.label}
        >
          <item.icon size={18} />
        </button>
      ))}

      <div className="activity-bar-divider" />

      <button
        className="activity-bar-btn"
        onClick={() => handlePanelClick('explorer')}
        title="Git"
      >
        <GitBranch size={18} />
      </button>

      <div className="activity-bar-spacer" />

      <button
        className="activity-bar-btn"
        onClick={toggleSidebar}
        title={sidebarVisible ? 'Close sidebar' : 'Open sidebar'}
      >
        <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
          {sidebarVisible ? (
            <path d="M10 4L6 8L10 12" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
          ) : (
            <path d="M6 4L10 8L6 12" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
          )}
        </svg>
      </button>
    </div>
  );
}