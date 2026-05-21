import { create } from 'zustand';
import type { UUID } from '@local-flow/shared-types';
import type { TerminalSession, TerminalOutput, CommandHistoryEntry } from '@local-flow/shared-types';

export interface TerminalStore {
  sessions: Partial<Record<UUID, TerminalSession>>;
  activeSessionId: UUID | null;
  outputs: Partial<Record<UUID, TerminalOutput[]>>;
  commandHistory: CommandHistoryEntry[];

  addSession: (session: TerminalSession) => void;
  removeSession: (sessionId: UUID) => void;
  setActiveSession: (sessionId: UUID | null) => void;
  appendOutput: (output: TerminalOutput) => void;
  addHistoryEntry: (entry: CommandHistoryEntry) => void;
  updateSessionStatus: (sessionId: UUID, status: TerminalSession['status']) => void;
  clearOutputs: (sessionId: UUID) => void;
}

export const useTerminalStore = create<TerminalStore>((set) => ({
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
    set((state) => ({
      sessions: Object.fromEntries(
        Object.entries(state.sessions).filter(([id]) => id !== sessionId)
      ),
      activeSessionId: state.activeSessionId === sessionId ? null : state.activeSessionId,
    }));
  },

  setActiveSession: (sessionId) => { set({ activeSessionId: sessionId }); },

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
