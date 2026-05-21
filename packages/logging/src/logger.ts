import type { EventSeverity, SystemEvent, UUID } from '@local-flow/shared-types';

export type LogLevel = EventSeverity;

const LOG_LEVELS: Record<LogLevel, number> = {
  debug: 0,
  info: 1,
  warning: 2,
  error: 3,
  critical: 4,
};

export interface LoggerConfig {
  minLevel: LogLevel;
  enableConsole: boolean;
  enableRemote: boolean;
  remoteEndpoint?: string;
  source: string;
}

export interface Transport {
  log(level: LogLevel, event: SystemEvent): void;
}

export class ConsoleTransport implements Transport {
  log(_level: LogLevel, event: SystemEvent): void {
    const prefix = `[${event.source}]`;
    switch (event.severity) {
      case 'debug':
        break;
      case 'info':
        console.info(prefix, event.message);
        break;
      case 'warning':
        console.warn(prefix, event.message, event.data ?? '');
        break;
      case 'error':
      case 'critical':
        console.error(prefix, event.message, event.error ?? '', event.data ?? '');
        break;
    }
  }
}

let eventCounter = 0;

function generateEventId(): UUID {
  eventCounter++;
  const ts = Date.now().toString(16);
  const cnt = eventCounter.toString(16).padStart(8, '0');
  return `${ts}-${cnt}-${crypto.randomUUID().slice(0, 8)}`;
}

export class Logger {
  private config: LoggerConfig;
  private transports: Transport[];

  constructor(config: Partial<LoggerConfig> = {}) {
    this.config = {
      minLevel: config.minLevel ?? 'info',
      enableConsole: config.enableConsole ?? true,
      enableRemote: config.enableRemote ?? false,
      source: config.source ?? 'app',
    };
    this.transports = [];
    if (this.config.enableConsole) {
      this.transports.push(new ConsoleTransport());
    }
  }

  private shouldLog(level: LogLevel): boolean {
    return LOG_LEVELS[level] >= LOG_LEVELS[this.config.minLevel];
  }

  private createEvent(severity: EventSeverity, type: string, message: string, data?: Record<string, unknown>): SystemEvent {
    return {
      id: generateEventId(),
      type,
      source: this.config.source,
      severity,
      timestamp: Date.now(),
      message,
      data,
    };
  }

  debug(type: string, message: string, data?: Record<string, unknown>): void {
    if (!this.shouldLog('debug')) return;
    const event = this.createEvent('debug', type, message, data);
    this.transports.forEach((t) => { t.log('debug', event); });
  }

  info(type: string, message: string, data?: Record<string, unknown>): void {
    if (!this.shouldLog('info')) return;
    const event = this.createEvent('info', type, message, data);
    this.transports.forEach((t) => { t.log('info', event); });
  }

  warn(type: string, message: string, data?: Record<string, unknown>): void {
    if (!this.shouldLog('warning')) return;
    const event = this.createEvent('warning', type, message, data);
    this.transports.forEach((t) => { t.log('warning', event); });
  }

  error(type: string, message: string, error?: Error, data?: Record<string, unknown>): void {
    if (!this.shouldLog('error')) return;
    const event = this.createEvent('error', type, message, {
      ...data,
      error: error
        ? {
            code: error.name,
            message: error.message,
            stack: error.stack,
          }
        : undefined,
    });
    this.transports.forEach((t) => { t.log('error', event); });
  }

  critical(type: string, message: string, error?: Error, data?: Record<string, unknown>): void {
    if (!this.shouldLog('critical')) return;
    const event = this.createEvent('critical', type, message, {
      ...data,
      error: error
        ? {
            code: error.name,
            message: error.message,
            stack: error.stack,
          }
        : undefined,
    });
    this.transports.forEach((t) => { t.log('critical', event); });
  }

  child(source: string): Logger {
    return new Logger({
      ...this.config,
      source: `${this.config.source}:${source}`,
    });
  }
}

export const rootLogger = new Logger({ source: 'localflow' });
