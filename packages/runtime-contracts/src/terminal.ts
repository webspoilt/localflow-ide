import { z } from 'zod';

export const CreateTerminalRequestSchema = z.object({
  cwd: z.string().optional(),
  columns: z.number().int().min(40).max(500).default(120),
  rows: z.number().int().min(10).max(200).default(40),
});

export const ResizeTerminalRequestSchema = z.object({
  sessionId: z.string().uuid(),
  columns: z.number().int().min(40).max(500),
  rows: z.number().int().min(10).max(200),
});

export const TerminalInputSchema = z.object({
  sessionId: z.string().uuid(),
  data: z.string().max(65536),
});

export const CloseTerminalRequestSchema = z.object({
  sessionId: z.string().uuid(),
});
