import { useRuntimeStore } from '@zynta/state';
import { GitBranch, Circle } from 'lucide-react';

export function StatusBar() {
  const activeTaskIds = useRuntimeStore((s) => s.activeTaskIds);
  const queueLength = useRuntimeStore((s) => s.queueLength);
  const failedTaskIds = useRuntimeStore((s) => s.failedTaskIds);

  const runtimeStatus = activeTaskIds.length > 0 ? 'running' : failedTaskIds.length > 0 ? 'error' : 'idle';

  return (
    <div className="status-bar">
      <div className="status-bar-left">
        <span className="status-item">
          <span className={`status-dot ${runtimeStatus}`} />
          {runtimeStatus === 'running' ? `${activeTaskIds.length} active` : runtimeStatus}
        </span>
        {queueLength > 0 && <span className="status-item">{queueLength} queued</span>}
        <span className="status-item">
          <GitBranch size={12} />
          main
        </span>
      </div>
      <div className="status-bar-right">
        <span className="status-item">UTF-8</span>
        <span className="status-item">Zynta Studio v0.1.0</span>
      </div>
    </div>
  );
}
