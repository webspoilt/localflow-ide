import { describe, it, expect } from 'vitest';
import { PluginAPI } from '../plugin';

describe('PluginAPI', () => {
  it('registers and lists plugins', () => {
    const api = new PluginAPI();
    const result = api.register({ name: 'test', version: '1.0.0', description: 'test plugin', permissions: [] });
    expect(result.ok).toBe(true);

    const plugins = api.list();
    expect(plugins).toHaveLength(1);
    expect(plugins[0].name).toBe('test');
  });

  it('prevents duplicate registration', () => {
    const api = new PluginAPI();
    api.register({ name: 'dup', version: '1.0.0', description: '', permissions: [] });
    const result = api.register({ name: 'dup', version: '2.0.0', description: '', permissions: [] });
    expect(result.ok).toBe(false);
    expect(result.error).toContain('already registered');
  });
});
