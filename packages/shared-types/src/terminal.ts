import type { UUID, Timestamp } from './index';

export interface TerminalSession {
  id: UUID;
  cwd: string;
  createdAt: Timestamp;
  lastActiveAt: Timestamp;
  status: 'active' | 'closed' | 'orphaned';
  columns: number;
  rows: number;
}

export interface TerminalOutput {
  sessionId: UUID;
  data: string;
  stream: 'stdout' | 'stderr';
  timestamp: Timestamp;
}

export interface TerminalCommand {
  sessionId: UUID;
  command: string;
  cwd: string;
  timestamp: Timestamp;
  exitCode?: number;
  duration?: number;
}

export interface CommandHistoryEntry {
  id: UUID;
  sessionId: UUID;
  command: string;
  cwd: string;
  timestamp: Timestamp;
  exitCode: number;
  duration: number;
  output: string;
}
