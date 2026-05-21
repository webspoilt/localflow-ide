import type { APIResponse } from './types';

export class TerminalAPI {
  constructor(private invoke: (cmd: string, args?: Record<string, unknown>) => Promise<unknown>) {}

  async create(): Promise<APIResponse<string>> {
    try {
      const sessionId = await this.invoke('create_terminal');
      return { ok: true, data: sessionId as string };
    } catch (e) {
      return { ok: false, error: String(e) };
    }
  }

  async write(sessionId: string, data: string): Promise<APIResponse<void>> {
    try {
      await this.invoke('terminal_write', { session_id: sessionId, data });
      return { ok: true };
    } catch (e) {
      return { ok: false, error: String(e) };
    }
  }

  async resize(sessionId: string, columns: number, rows: number): Promise<APIResponse<void>> {
    try {
      await this.invoke('terminal_resize', { session_id: sessionId, columns, rows });
      return { ok: true };
    } catch (e) {
      return { ok: false, error: String(e) };
    }
  }

  async close(sessionId: string): Promise<APIResponse<boolean>> {
    try {
      const result = await this.invoke('close_terminal', { session_id: sessionId });
      return { ok: true, data: result as boolean };
    } catch (e) {
      return { ok: false, error: String(e) };
    }
  }

  async list(): Promise<APIResponse<string[]>> {
    try {
      const result = await this.invoke('list_terminals');
      return { ok: true, data: result as string[] };
    } catch (e) {
      return { ok: false, error: String(e) };
    }
  }
}
