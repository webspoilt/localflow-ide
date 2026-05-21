import type { APIResponse } from './types';

export interface PluginManifest {
  name: string;
  version: string;
  description: string;
  permissions: string[];
}

export class PluginAPI {
  private plugins = new Map<string, PluginManifest>();

  register(manifest: PluginManifest): APIResponse<void> {
    if (this.plugins.has(manifest.name)) {
      return { ok: false, error: `Plugin '${manifest.name}' already registered` };
    }
    this.plugins.set(manifest.name, manifest);
    return { ok: true };
  }

  list(): PluginManifest[] {
    return Array.from(this.plugins.values());
  }

  get(name: string): PluginManifest | undefined {
    return this.plugins.get(name);
  }
}
