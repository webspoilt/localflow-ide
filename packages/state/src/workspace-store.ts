import { create } from 'zustand';
import type { UUID, Timestamp } from '@local-flow/shared-types';

export interface FileNode {
  id: UUID;
  name: string;
  path: string;
  type: 'file' | 'directory';
  children?: FileNode[];
  size?: number;
  modifiedAt?: Timestamp;
  language?: string;
}

export interface EditorTab {
  id: UUID;
  filePath: string;
  fileName: string;
  language: string;
  isDirty: boolean;
  cursorPosition: { line: number; column: number };
}

export interface WorkspaceStore {
  root: FileNode | null;
  openTabs: EditorTab[];
  activeTabId: UUID | null;
  selectedFilePath: string | null;

  setRoot: (root: FileNode) => void;
  openFile: (file: FileNode) => void;
  closeTab: (tabId: UUID) => void;
  setActiveTab: (tabId: UUID | null) => void;
  selectFile: (filePath: string | null) => void;
  markTabDirty: (tabId: UUID, dirty: boolean) => void;
  updateCursorPosition: (tabId: UUID, line: number, column: number) => void;
}

export const useWorkspaceStore = create<WorkspaceStore>((set) => ({
  root: null,
  openTabs: [],
  activeTabId: null,
  selectedFilePath: null,

  setRoot: (root) => { set({ root }); },

  openFile: (file) => {
    set((state) => {
      const existing = state.openTabs.find((t) => t.filePath === file.path);
      if (existing) {
        return { activeTabId: existing.id, selectedFilePath: file.path };
      }
      const newTab: EditorTab = {
        id: crypto.randomUUID(),
        filePath: file.path,
        fileName: file.name,
        language: file.language ?? 'plaintext',
        isDirty: false,
        cursorPosition: { line: 0, column: 0 },
      };
      return {
        openTabs: [...state.openTabs, newTab],
        activeTabId: newTab.id,
        selectedFilePath: file.path,
      };
    });
  },

  closeTab: (tabId) => {
    set((state) => {
      const tabs = state.openTabs.filter((t) => t.id !== tabId);
      const activeTabId =
        state.activeTabId === tabId ? (tabs.length > 0 ? tabs[tabs.length - 1].id : null) : state.activeTabId;
      return { openTabs: tabs, activeTabId };
    });
  },

  setActiveTab: (tabId) => { set({ activeTabId: tabId }); },

  selectFile: (filePath) => { set({ selectedFilePath: filePath }); },

  markTabDirty: (tabId, dirty) => {
    set((state) => ({
      openTabs: state.openTabs.map((t) => (t.id === tabId ? { ...t, isDirty: dirty } : t)),
    }));
  },

  updateCursorPosition: (tabId, line, column) => {
    set((state) => ({
      openTabs: state.openTabs.map((t) =>
        t.id === tabId ? { ...t, cursorPosition: { line, column } } : t,
      ),
    }));
  },
}));
