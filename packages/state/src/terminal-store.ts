import { create } from 'zustand';
import type { UUID } from '@zynta/shared-types';
import type { TerminalSession, TerminalOutput, CommandHistoryEntry } from '@zynta/shared-types';

export interface TerminalStore {
  sessions: Record<UUID, TerminalSession>;
  activeSessionId: UUID | null;
  outputs: Record<UUID, TerminalOutput[]>;
  commandHistory: CommandHistoryEntry[];

  addSession: (session: TerminalSession) => void;
  removeSession: (sessionId: UUID) => void;
  setActiveSession: (sessionId: UUID | null) => void;
  appendOutput: (output: TerminalOutput) => void;
  addHistoryEntry: (entry: CommandHistoryEntry) => void;
  updateSessionStatus: (sessionId: UUID, status: TerminalSession['status']) => void;
  clearOutputs: (sessionId: UUID) => void;
}

export const useTerminalStore = create<TerminalStore>((set, get) => ({
  sessions: {},
  activeSessionId: null,
  outputs: {},
  commandHistory: [],

  addSession: (session) => {
    set((state) => ({
      sessions: { ...state.sessions, [session.id]: session },
      outputs: { ...state.outputs, [session.id]: [] },
    }));
  },

  removeSession: (sessionId) => {
    set((state) => {
      const { [sessionId]: _removedSession, ...remainingSessions } = state.sessions;
      const { [sessionId]: _removedOutputs, ...remainingOutputs } = state.outputs;
      return {
        sessions: remainingSessions,
        outputs: remainingOutputs,
        activeSessionId: state.activeSessionId === sessionId ? null : state.activeSessionId,
      };
    });
  },

  setActiveSession: (sessionId) => set({ activeSessionId: sessionId }),

  appendOutput: (output) => {
    set((state) => {
      const sessionOutputs = state.outputs[output.sessionId] ?? [];
      return {
        outputs: {
          ...state.outputs,
          [output.sessionId]: [...sessionOutputs, output],
        },
      };
    });
  },

  addHistoryEntry: (entry) => {
    set((state) => ({
      commandHistory: [...state.commandHistory, entry],
    }));
  },

  updateSessionStatus: (sessionId, status) => {
    set((state) => {
      const session = state.sessions[sessionId];
      if (!session) return state;
      return {
        sessions: {
          ...state.sessions,
          [sessionId]: {
            ...session,
            status,
            lastActiveAt: Date.now(),
          },
        },
      };
    });
  },

  clearOutputs: (sessionId) => {
    set((state) => ({
      outputs: {
        ...state.outputs,
        [sessionId]: [],
      },
    }));
  },
}));
