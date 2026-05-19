import type { UUID, Timestamp } from './index';

export type TaskStatus =
  | 'pending'
  | 'queued'
  | 'running'
  | 'paused'
  | 'completed'
  | 'failed'
  | 'cancelled'
  | 'timeout';

export type TaskPriority = 'low' | 'normal' | 'high' | 'critical';

export type TaskType =
  | 'command'
  | 'build'
  | 'test'
  | 'deploy'
  | 'lint'
  | 'analysis'
  | 'ai_generate'
  | 'custom';

export interface TaskDefinition {
  type: TaskType;
  command?: string;
  args?: string[];
  cwd?: string;
  env?: Record<string, string>;
  timeout?: number;
  retry?: RetryPolicy;
  sandbox?: SandboxPolicy;
}

export interface RetryPolicy {
  maxAttempts: number;
  backoffMs: number;
  backoffMultiplier?: number;
}

export interface SandboxPolicy {
  allowNetwork: boolean;
  allowFilesystem: boolean;
  allowedPaths?: string[];
  allowedCommands?: string[];
}

export interface Task {
  id: UUID;
  correlationId?: UUID;
  status: TaskStatus;
  type: TaskType;
  definition: TaskDefinition;
  priority: TaskPriority;
  createdAt: Timestamp;
  startedAt?: Timestamp;
  completedAt?: Timestamp;
  attempts: number;
  error?: TaskError;
  result?: TaskResult;
  metadata?: Record<string, string>;
}

export interface TaskError {
  code: string;
  message: string;
  stack?: string;
  exitCode?: number;
}

export interface TaskResult {
  exitCode: number;
  stdout: string;
  stderr: string;
  duration: number;
}

export interface TaskEvent {
  taskId: UUID;
  type: 'created' | 'queued' | 'started' | 'progress' | 'completed' | 'failed' | 'cancelled';
  timestamp: Timestamp;
  data?: unknown;
}
