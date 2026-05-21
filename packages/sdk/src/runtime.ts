import type { APIResponse } from './types';
import type { HealthResponse, TaskResponse } from '@local-flow/shared-types';

export class RuntimeAPI {
  constructor(private invoke: (cmd: string, args?: Record<string, unknown>) => Promise<unknown>) {}

  async health(): Promise<APIResponse<HealthResponse>> {
    try {
      const result = await this.invoke('health');
      return { ok: true, data: result as HealthResponse };
    } catch (e) {
      return { ok: false, error: String(e) };
    }
  }

  async executeTask(command: string, cwd?: string): Promise<APIResponse<TaskResponse>> {
    try {
      const result = await this.invoke('execute_task', { command, cwd });
      return { ok: true, data: result as TaskResponse };
    } catch (e) {
      return { ok: false, error: String(e) };
    }
  }

  async cancelTask(taskId: string): Promise<APIResponse<boolean>> {
    try {
      const result = await this.invoke('cancel_task', { task_id: taskId });
      return { ok: true, data: result as boolean };
    } catch (e) {
      return { ok: false, error: String(e) };
    }
  }
}
