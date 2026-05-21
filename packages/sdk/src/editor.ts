import type { APIResponse } from './types';

export class EditorAPI {
  constructor(private invoke: (cmd: string, args?: Record<string, unknown>) => Promise<unknown>) {}

  async openFile(path: string): Promise<APIResponse<void>> {
    try {
      await this.invoke('read_directory', { path });
      return { ok: true };
    } catch (e) {
      return { ok: false, error: String(e) };
    }
  }
}
