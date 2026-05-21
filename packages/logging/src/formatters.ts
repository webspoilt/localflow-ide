import type { SystemEvent, Task, TaskEvent } from '@local-flow/shared-types';

export function formatTaskEvent(event: TaskEvent): string {
  const ts = new Date(event.timestamp).toISOString();
  return `[${ts}] [${event.type.toUpperCase()}] task=${event.taskId.slice(0, 8)}`;
}

export function formatTaskStatus(task: Task): string {
  const elapsed = task.startedAt ? Date.now() - task.startedAt : 0;
  const status = task.status.toUpperCase().padEnd(10);
  const id = task.id.slice(0, 8);
  const type = task.type.padEnd(12);
  return `${status} | ${id} | ${type} | attempts=${String(task.attempts)} | elapsed=${String(elapsed)}ms`;
}

export function formatSystemEvent(event: SystemEvent): string {
  const ts = new Date(event.timestamp).toISOString();
  const sev = event.severity.toUpperCase().padEnd(8);
  return `[${ts}] ${sev} [${event.source}] ${event.message}`;
}

export function formatDuration(ms: number): string {
  if (ms < 1000) return `${String(ms)}ms`;
  if (ms < 60000) return `${(ms / 1000).toFixed(1)}s`;
  const min = Math.floor(ms / 60000);
  const sec = Math.floor((ms % 60000) / 1000);
  return `${String(min)}m ${String(sec)}s`;
}
