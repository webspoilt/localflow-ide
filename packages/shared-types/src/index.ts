export * from './task';
export * from './runtime';
export * from './terminal';
export * from './events';

export type UUID = string;
export type Timestamp = number;

export interface HealthStatus {
  status: 'healthy' | 'degraded' | 'unhealthy';
  uptime: number;
  version: string;
}
