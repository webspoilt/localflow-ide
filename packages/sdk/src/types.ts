export interface CapabilityRequest {
  capability: 'filesystem' | 'network' | 'terminal' | 'model' | 'admin';
  reason: string;
}

export interface APIResponse<T> {
  ok: boolean;
  data?: T;
  error?: string;
}
