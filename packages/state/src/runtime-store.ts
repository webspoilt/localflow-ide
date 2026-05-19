import { create } from 'zustand';
import type { Task, UUID, RuntimeAction, RuntimeConfig, WorkerInfo } from '@zynta/shared-types';

export interface RuntimeStore {
  tasks: Record<UUID, Task>;
  taskOrder: UUID[];
  activeTaskIds: UUID[];
  completedTaskIds: UUID[];
  failedTaskIds: UUID[];
  cancelledTaskIds: UUID[];
  workers: Record<UUID, WorkerInfo>;
  queueLength: number;
  config: RuntimeConfig;

  dispatch: (action: RuntimeAction) => void;
  setConfig: (config: Partial<RuntimeConfig>) => void;
  getTaskById: (id: UUID) => Task | undefined;
  getActiveTasks: () => Task[];
  clearCompleted: () => void;
  reset: () => void;
}

const initialState = {
  tasks: {},
  taskOrder: [],
  activeTaskIds: [],
  completedTaskIds: [],
  failedTaskIds: [],
  cancelledTaskIds: [],
  workers: {},
  queueLength: 0,
  config: {
    maxConcurrent: 4,
    defaultTimeout: 30000,
    enableSandbox: true,
    allowedCommands: [],
    deniedCommands: [],
  },
};

export const useRuntimeStore = create<RuntimeStore>((set, get) => ({
  ...initialState,

  dispatch: (action: RuntimeAction) => {
    switch (action.type) {
      case 'TASK_CREATED': {
        set((state) => ({
          tasks: { ...state.tasks, [action.task.id]: action.task },
          taskOrder: [...state.taskOrder, action.task.id],
        }));
        break;
      }
      case 'TASK_QUEUED': {
        set((state) => {
          const task = state.tasks[action.taskId];
          if (!task) return state;
          return {
            tasks: {
              ...state.tasks,
              [action.taskId]: { ...task, status: 'queued' as const },
            },
            queueLength: state.queueLength + 1,
          };
        });
        break;
      }
      case 'TASK_STARTED': {
        set((state) => {
          const task = state.tasks[action.taskId];
          if (!task) return state;
          return {
            tasks: {
              ...state.tasks,
              [action.taskId]: {
                ...task,
                status: 'running' as const,
                startedAt: Date.now(),
              },
            },
            activeTaskIds: [...state.activeTaskIds.filter((id) => id !== action.taskId), action.taskId],
            queueLength: Math.max(0, state.queueLength - 1),
            workers: {
              ...state.workers,
              [action.workerId]: {
                id: action.workerId,
                status: 'busy' as const,
                currentTaskId: action.taskId,
                startedAt: Date.now(),
                tasksProcessed: (state.workers[action.workerId]?.tasksProcessed ?? 0),
              },
            },
          };
        });
        break;
      }
      case 'TASK_PROGRESS': {
        set((state) => {
          const task = state.tasks[action.taskId];
          if (!task) return state;
          return {
            tasks: {
              ...state.tasks,
              [action.taskId]: {
                ...task,
                metadata: {
                  ...task.metadata,
                  progress: String(action.progress),
                  progressMessage: action.message,
                },
              },
            },
          };
        });
        break;
      }
      case 'TASK_COMPLETED': {
        set((state) => {
          const task = state.tasks[action.taskId];
          if (!task) return state;
          return {
            tasks: {
              ...state.tasks,
              [action.taskId]: {
                ...task,
                status: 'completed' as const,
                completedAt: Date.now(),
                result: action.result,
              },
            },
            activeTaskIds: state.activeTaskIds.filter((id) => id !== action.taskId),
            completedTaskIds: [...state.completedTaskIds, action.taskId],
          };
        });
        break;
      }
      case 'TASK_FAILED': {
        set((state) => {
          const task = state.tasks[action.taskId];
          if (!task) return state;
          return {
            tasks: {
              ...state.tasks,
              [action.taskId]: {
                ...task,
                status: 'failed' as const,
                completedAt: Date.now(),
                error: action.error,
              },
            },
            activeTaskIds: state.activeTaskIds.filter((id) => id !== action.taskId),
            failedTaskIds: [...state.failedTaskIds, action.taskId],
          };
        });
        break;
      }
      case 'TASK_CANCELLED': {
        set((state) => {
          const task = state.tasks[action.taskId];
          if (!task) return state;
          return {
            tasks: {
              ...state.tasks,
              [action.taskId]: {
                ...task,
                status: 'cancelled' as const,
                completedAt: Date.now(),
              },
            },
            activeTaskIds: state.activeTaskIds.filter((id) => id !== action.taskId),
            cancelledTaskIds: [...state.cancelledTaskIds, action.taskId],
          };
        });
        break;
      }
      case 'WORKER_STATUS': {
        set((state) => ({
          workers: {
            ...state.workers,
            [action.workerId]: {
              ...state.workers[action.workerId],
              id: action.workerId,
              status: action.status,
              currentTaskId: action.status === 'idle' ? undefined : state.workers[action.workerId]?.currentTaskId,
              tasksProcessed:
                action.status === 'idle'
                  ? (state.workers[action.workerId]?.tasksProcessed ?? 0) + 1
                  : (state.workers[action.workerId]?.tasksProcessed ?? 0),
            },
          },
        }));
        break;
      }
      case 'QUEUE_UPDATE': {
        set({ queueLength: action.length });
        break;
      }
    }
  },

  setConfig: (partial) => {
    set((state) => ({
      config: { ...state.config, ...partial },
    }));
  },

  getTaskById: (id) => {
    return get().tasks[id];
  },

  getActiveTasks: () => {
    const state = get();
    return state.activeTaskIds.map((id) => state.tasks[id]).filter(Boolean);
  },

  clearCompleted: () => {
    set((state) => {
      const newTasks = { ...state.tasks };
      for (const id of [...state.completedTaskIds, ...state.failedTaskIds, ...state.cancelledTaskIds]) {
        delete newTasks[id];
      }
      return {
        tasks: newTasks,
        completedTaskIds: [],
        failedTaskIds: [],
        cancelledTaskIds: [],
        taskOrder: state.taskOrder.filter(
          (id) =>
            !state.completedTaskIds.includes(id) &&
            !state.failedTaskIds.includes(id) &&
            !state.cancelledTaskIds.includes(id),
        ),
      };
    });
  },

  reset: () => set(initialState),
}));
