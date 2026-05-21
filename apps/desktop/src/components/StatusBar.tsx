import { useRuntimeStore } from '@local-flow/state';

export function StatusBar() {
  const activeTaskIds = useRuntimeStore((s) => s.activeTaskIds);
  const queueLength = useRuntimeStore((s) => s.queueLength);
  const failedTaskIds = useRuntimeStore((s) => s.failedTaskIds);

  const status: 'running' | 'idle' | 'error' =
    activeTaskIds.length > 0 ? 'running' :
    failedTaskIds.length > 0 ? 'error' : 'idle';

  return (
    <div className="status-bar">
      <div className="status-bar-left">
        <span className="status-item">
          <span className={`status-dot ${status}`} />
          {status === 'running'
            ? `${String(activeTaskIds.length)} active`
            : status === 'error'
            ? `${String(failedTaskIds.length)} failed`
            : 'Ready'}
        </span>
        {queueLength > 0 && (
          <span className="status-item">{queueLength} queued</span>
        )}
      </div>
      <div className="status-bar-right">
        <span className="status-item">LocalFlow IDE</span>
      </div>
    </div>
  );
}