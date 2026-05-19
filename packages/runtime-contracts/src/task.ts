import { z } from 'zod';

export const RetryPolicySchema = z.object({
  maxAttempts: z.number().int().min(1).max(10).default(3),
  backoffMs: z.number().int().min(100).max(60000).default(1000),
  backoffMultiplier: z.number().min(1).max(10).default(2).optional(),
});

export const SandboxPolicySchema = z.object({
  allowNetwork: z.boolean().default(false),
  allowFilesystem: z.boolean().default(true),
  allowedPaths: z.array(z.string()).optional(),
  allowedCommands: z.array(z.string()).optional(),
});

export const TaskDefinitionSchema = z.object({
  type: z.enum(['command', 'build', 'test', 'deploy', 'lint', 'analysis', 'ai_generate', 'custom']),
  command: z.string().optional(),
  args: z.array(z.string()).optional(),
  cwd: z.string().optional(),
  env: z.record(z.string(), z.string()).optional(),
  timeout: z.number().int().min(1000).max(3600000).optional(),
  retry: RetryPolicySchema.optional(),
  sandbox: SandboxPolicySchema.optional(),
});

export const ExecuteTaskRequestSchema = z.object({
  definition: TaskDefinitionSchema,
  priority: z.enum(['low', 'normal', 'high', 'critical']).default('normal'),
  correlationId: z.string().uuid().optional(),
  metadata: z.record(z.string(), z.string()).optional(),
});

export const CancelTaskRequestSchema = z.object({
  taskId: z.string().uuid(),
  reason: z.string().optional(),
});

export const GetTaskStatusRequestSchema = z.object({
  taskId: z.string().uuid(),
});
