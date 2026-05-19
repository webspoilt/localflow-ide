import { useRuntimeStore, useTerminalStore } from '@zynta/state';
import { GitBranch, Circle } from 'lucide-react';

export function StatusBar() {
  const activeTaskIds = useRuntimeStore((s) => s.activeTaskIds);
  const queueLength = useRuntimeStore((s) => s.queueLength);
  const failedTaskIds = useRuntimeStore((s) => s.failedTaskIds);
  const sessions = useTerminalStore((s) => s.sessions);
  const sessionCount = Object.keys(sessions).length;

  const status: 'running' | 'idle' | 'error' =
    activeTaskIds.length > 0 ? 'running' :
    failedTaskIds.length > 0 ? 'error' : 'idle';

  return (
    <div className="status-bar">
      <div className="status-bar-left">
        <span className="status-item">
          <span className={`status-dot ${status}`} />
          {status === 'running'
            ? `${activeTaskIds.length} active`
            : status === 'error'
            ? `${failedTaskIds.length} failed`
            : 'Ready'}
        </span>
        {queueLength > 0 && (
          <span className="status-item">{queueLength} queued</span>
        )}
        <span className="status-item">
          <GitBranch size={11} />
          main
        </span>
      </div>
      <div className="status-bar-right">
        <span className="status-item">UTF-8</span>
        <span className="status-item">TypeScript</span>
        <span className="status-item">Zynta v0.2.0</span>
      </div>
    </div>
  );
}