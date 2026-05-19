import { create } from 'zustand';

export type PanelId = 'explorer' | 'search' | 'terminal' | 'output' | 'extensions' | 'settings';
export type TerminalPanelPosition = 'bottom' | 'right' | 'collapsed';

export interface PanelState {
  id: PanelId;
  visible: boolean;
  width?: number;
  height?: number;
}

export interface UIStore {
  sidebarVisible: boolean;
  sidebarWidth: number;
  activePanel: PanelId;
  terminalPosition: TerminalPanelPosition;
  terminalHeight: number;
  panels: Record<PanelId, PanelState>;

  toggleSidebar: () => void;
  setSidebarWidth: (width: number) => void;
  setActivePanel: (panel: PanelId) => void;
  setTerminalPosition: (position: TerminalPanelPosition) => void;
  setTerminalHeight: (height: number) => void;
  togglePanel: (panel: PanelId) => void;
  setPanelVisibility: (panel: PanelId, visible: boolean) => void;
}

export const useUIStore = create<UIStore>((set) => ({
  sidebarVisible: true,
  sidebarWidth: 260,
  activePanel: 'explorer',
  terminalPosition: 'bottom',
  terminalHeight: 250,
  panels: {
    explorer: { id: 'explorer', visible: true },
    search: { id: 'search', visible: false },
    terminal: { id: 'terminal', visible: true },
    output: { id: 'output', visible: true },
    extensions: { id: 'extensions', visible: false },
    settings: { id: 'settings', visible: false },
  },

  toggleSidebar: () => set((state) => ({ sidebarVisible: !state.sidebarVisible })),
  setSidebarWidth: (width) => set({ sidebarWidth: width }),
  setActivePanel: (panel) => set({ activePanel: panel }),
  setTerminalPosition: (position) => set({ terminalPosition: position }),
  setTerminalHeight: (height) => set({ terminalHeight: height }),

  togglePanel: (panel) =>
    set((state) => ({
      panels: {
        ...state.panels,
        [panel]: { ...state.panels[panel], visible: !state.panels[panel].visible },
      },
    })),

  setPanelVisibility: (panel, visible) =>
    set((state) => ({
      panels: {
        ...state.panels,
        [panel]: { ...state.panels[panel], visible },
      },
    })),
}));
