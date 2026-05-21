import { describe, it, expect, vi } from 'vitest';
import { RuntimeAPI } from '../runtime';

describe('RuntimeAPI', () => {
  it('returns health data on success', async () => {
    const mockInvoke = vi.fn().mockResolvedValue({
      status: 'healthy',
      version: '0.1.0',
      uptime: 42,
      active_tasks: 0,
      queue_length: 0,
    });

    const api = new RuntimeAPI(mockInvoke);
    const result = await api.health();

    expect(result.ok).toBe(true);
    expect(result.data?.status).toBe('healthy');
    expect(mockInvoke).toHaveBeenCalledWith('health');
  });

  it('returns error on invoke failure', async () => {
    const mockInvoke = vi.fn().mockRejectedValue(new Error('connection refused'));
    const api = new RuntimeAPI(mockInvoke);
    const result = await api.health();

    expect(result.ok).toBe(false);
    expect(result.error).toContain('connection refused');
  });
});
