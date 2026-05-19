import { z } from 'zod';
import { ExecuteTaskRequestSchema, CancelTaskRequestSchema, GetTaskStatusRequestSchema } from './task';
import {
  CreateTerminalRequestSchema,
  ResizeTerminalRequestSchema,
  TerminalInputSchema,
  CloseTerminalRequestSchema,
} from './terminal';

export const IpcRequestSchema = z.discriminatedUnion('method', [
  // Task commands
  z.object({ method: z.literal('execute_task'), params: ExecuteTaskRequestSchema }),
  z.object({ method: z.literal('cancel_task'), params: CancelTaskRequestSchema }),
  z.object({ method: z.literal('get_task_status'), params: GetTaskStatusRequestSchema }),
  // Terminal commands
  z.object({ method: z.literal('create_terminal'), params: CreateTerminalRequestSchema }),
  z.object({ method: z.literal('resize_terminal'), params: ResizeTerminalRequestSchema }),
  z.object({ method: z.literal('terminal_input'), params: TerminalInputSchema }),
  z.object({ method: z.literal('close_terminal'), params: CloseTerminalRequestSchema }),
  // System commands
  z.object({ method: z.literal('get_health'), params: z.object({}).optional() }),
  z.object({ method: z.literal('get_runtime_state'), params: z.object({}).optional() }),
  z.object({ method: z.literal('shutdown'), params: z.object({ reason: z.string().optional() }).optional() }),
]);

export type IpcRequest = z.infer<typeof IpcRequestSchema>;

export interface IpcSuccessResponse<T = unknown> {
  ok: true;
  data: T;
  requestId: string;
}

export interface IpcErrorResponse {
  ok: false;
  error: {
    code: string;
    message: string;
  };
  requestId: string;
}

export type IpcResponse<T = unknown> = IpcSuccessResponse<T> | IpcErrorResponse;
