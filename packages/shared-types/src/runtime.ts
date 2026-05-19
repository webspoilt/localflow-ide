import type { UUID, Timestamp } from './index';
import type { Task } from './task';

export interface RuntimeState {
  tasks: Record<UUID, Task>;
  activeTaskIds: UUID[];
  completedTaskIds: UUID[];
  failedTaskIds: UUID[];
  queueLength: number;
  workerCount: number;
  uptime: number;
}

export interface WorkerInfo {
  id: UUID;
  status: 'idle' | 'busy' | 'error';
  currentTaskId?: UUID;
  startedAt: Timestamp;
  tasksProcessed: number;
}

export interface RuntimeConfig {
  maxConcurrent: number;
  defaultTimeout: number;
  enableSandbox: boolean;
  allowedCommands: string[];
  deniedCommands: string[];
}

export type RuntimeAction =
  | { type: 'TASK_CREATED'; task: Task }
  | { type: 'TASK_QUEUED'; taskId: UUID }
  | { type: 'TASK_STARTED'; taskId: UUID; workerId: UUID }
  | { type: 'TASK_PROGRESS'; taskId: UUID; progress: number; message: string }
  | { type: 'TASK_COMPLETED'; taskId: UUID; result: Task['result'] }
  | { type: 'TASK_FAILED'; taskId: UUID; error: Task['error'] }
  | { type: 'TASK_CANCELLED'; taskId: UUID; reason?: string }
  | { type: 'WORKER_STATUS'; workerId: UUID; status: WorkerInfo['status'] }
  | { type: 'QUEUE_UPDATE'; length: number };
