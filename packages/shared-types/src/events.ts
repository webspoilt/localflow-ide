import type { UUID, Timestamp } from './index';

export type EventSeverity = 'debug' | 'info' | 'warning' | 'error' | 'critical';

export interface SystemEvent {
  id: UUID;
  type: string;
  source: string;
  severity: EventSeverity;
  timestamp: Timestamp;
  message: string;
  data?: Record<string, unknown>;
  error?: {
    code: string;
    message: string;
    stack?: string;
  };
}

export type EventHandler = (event: SystemEvent) => void;

export interface EventSubscription {
  types?: string[];
  sources?: string[];
  minSeverity?: EventSeverity;
  handler: EventHandler;
}
