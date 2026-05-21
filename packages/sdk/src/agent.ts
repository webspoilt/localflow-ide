import type { APIResponse } from './types';

export class AgentAPI {
  constructor(private invoke: (cmd: string, args?: Record<string, unknown>) => Promise<unknown>) {}

  async execute(task: string): Promise<APIResponse<string>> {
    try {
      const result = await this.invoke('execute_task', { command: task });
      return { ok: true, data: JSON.stringify(result) };
    } catch (e) {
      return { ok: false, error: String(e) };
    }
  }
}
