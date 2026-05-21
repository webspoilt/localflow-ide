import { useEffect } from 'react';
import { useRuntimeStore, useTerminalStore } from '@local-flow/state';
import type { TaskType, TaskPriority } from '@local-flow/shared-types';
import { rootLogger } from '@local-flow/logging';

const logger = rootLogger.child('runtime-events');

export function useRuntimeEvents() {
  const dispatch = useRuntimeStore((s) => s.dispatch);
  const appendOutput = useTerminalStore((s) => s.appendOutput);

  useEffect(() => {
    const unlisten: (() => void)[] = [];

    async function setup() {
      try {
        const { listen } = await import('@tauri-apps/api/event');

        interface RuntimeEventPayload {
          taskId?: string;
          definition?: {
            type?: string;
            command?: string;
            args?: string[];
            cwd?: string;
            env?: Record<string, string>;
            timeout_ms?: number;
            priority?: string;
          };
          progress?: number;
          message?: string;
          exitCode?: number;
          stdout?: string;
          stderr?: string;
          error?: string;
        }

        const unlistenTaskEvent = await listen<unknown>('runtime:event', (event) => {
          const rawEvent = event.payload as Record<string, unknown> | null | undefined;
          if (!rawEvent || typeof rawEvent !== 'object' || typeof rawEvent.type !== 'string') return;

          const type = rawEvent.type;
          const payload = (rawEvent.payload ?? {}) as RuntimeEventPayload;

          const state = useRuntimeStore.getState();

          const ensureTaskExists = (taskId: string) => {
            if (!state.tasks[taskId]) {
              state.dispatch({
                type: 'TASK_CREATED',
                task: {
                  id: taskId,
                  status: 'pending',
                  type: 'command',
                  definition: {
                    type: 'command',
                    command: 'Task ' + taskId.slice(0, 8),
                  },
                  priority: 'normal',
                  createdAt: Date.now(),
                  attempts: 1,
                },
              });
            }
          };

          switch (type) {
            case 'TASK_CREATED': {
              const definition = payload.definition ?? {};
              if (payload.taskId) {
                state.dispatch({
                  type: 'TASK_CREATED',
                  task: {
                    id: payload.taskId,
                    status: 'pending',
                    type: (definition.type ?? 'command') as TaskType,
                    definition: {
                      type: (definition.type ?? 'command') as TaskType,
                      command: definition.command ?? '',
                      args: definition.args ?? [],
                      cwd: definition.cwd ?? '',
                      env: definition.env ?? {},
                      timeout: definition.timeout_ms ?? 30000,
                    },
                    priority: (definition.priority ?? 'normal') as TaskPriority,
                    createdAt: Date.now(),
                    attempts: 1,
                  },
                });
              }
              break;
            }
            case 'TASK_QUEUED': {
              if (payload.taskId) {
                ensureTaskExists(payload.taskId);
                state.dispatch({ type: 'TASK_QUEUED', taskId: payload.taskId });
              }
              break;
            }
            case 'TASK_STARTED': {
              if (payload.taskId) {
                ensureTaskExists(payload.taskId);
                state.dispatch({ type: 'TASK_STARTED', taskId: payload.taskId, workerId: 'supervisor' });
              }
              break;
            }
            case 'TASK_PROGRESS': {
              if (payload.taskId) {
                ensureTaskExists(payload.taskId);
                state.dispatch({
                  type: 'TASK_PROGRESS',
                  taskId: payload.taskId,
                  progress: payload.progress ?? 0,
                  message: payload.message ?? '',
                });
              }
              break;
            }
            case 'TASK_COMPLETED': {
              if (payload.taskId) {
                ensureTaskExists(payload.taskId);
                state.dispatch({
                  type: 'TASK_COMPLETED',
                  taskId: payload.taskId,
                  result: {
                    exitCode: payload.exitCode ?? 0,
                    stdout: payload.stdout ?? '',
                    stderr: payload.stderr ?? '',
                    duration: 0,
                  },
                });
              }
              break;
            }
            case 'TASK_FAILED': {
              if (payload.taskId) {
                ensureTaskExists(payload.taskId);
                state.dispatch({
                  type: 'TASK_FAILED',
                  taskId: payload.taskId,
                  error: {
                    code: 'EXEC_ERROR',
                    message: payload.error ?? 'Execution failed',
                    exitCode: 1,
                  },
                });
              }
              break;
            }
            case 'TASK_CANCELLED': {
              if (payload.taskId) {
                ensureTaskExists(payload.taskId);
                state.dispatch({ type: 'TASK_CANCELLED', taskId: payload.taskId });
              }
              break;
            }
            default:
              break;
          }
        });
        unlisten.push(unlistenTaskEvent);

        const unlistenLog = await listen('runtime:log', (event) => {
          const payload = event.payload as { sessionId: string; data: string; stream: string; timestamp?: number };
          appendOutput({
            sessionId: payload.sessionId,
            data: payload.data,
            stream: payload.stream as 'stdout' | 'stderr',
            timestamp: payload.timestamp ?? Date.now(),
          });
        });
        unlisten.push(unlistenLog);

        logger.info('runtime_connected', 'Connected to runtime event bus');
      } catch (err) {
        logger.warn('runtime_connection_failed', 'Runtime event bus not available (Tauri backend not running)', {
          error: String(err),
        });
      }
    }

    setup().catch((err: unknown) => {
      logger.warn('runtime_setup_failed', 'Failed to set up runtime event listeners', { error: String(err) });
    });

    return () => {
      unlisten.forEach((fn) => { fn(); });
    };
  }, [dispatch, appendOutput]);
}
