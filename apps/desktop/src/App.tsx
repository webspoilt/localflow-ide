import { useCallback } from 'react';
import { useUIStore, useTerminalStore } from '@zynta/state';
import { ActivityBar } from './components/ActivityBar';
import { Sidebar } from './components/Sidebar';
import { MainArea } from './components/MainArea';
import { TerminalPanel } from './components/TerminalPanel';
import { StatusBar } from './components/StatusBar';
import { ErrorBoundary } from './components/ErrorBoundary';
import { useRuntimeEvents } from './hooks/useRuntimeEvents';

export function App() {
  useRuntimeEvents();

  const sidebarVisible = useUIStore((s) => s.sidebarVisible);
  const terminalPosition = useUIStore((s) => s.terminalPosition);

  const layoutClass = [
    'app-shell',
    sidebarVisible ? 'sidebar-open' : '',
    terminalPosition === 'collapsed' ? 'terminal-hidden' : '',
    terminalPosition === 'right' ? 'terminal-right' : '',
  ]
    .filter(Boolean)
    .join(' ');

  return (
    <ErrorBoundary>
      <div className={layoutClass}>
        <ActivityBar />
        {sidebarVisible && <Sidebar />}
        <MainArea />
        {terminalPosition !== 'collapsed' && <TerminalPanel />}
        <StatusBar />
      </div>
    </ErrorBoundary>
  );
}