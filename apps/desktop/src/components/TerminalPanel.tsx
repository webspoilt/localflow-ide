import { useRef, useEffect, useCallback, useState } from 'react';
import { Terminal as XTerm } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import '@xterm/xterm/css/xterm.css';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useUIStore } from '@local-flow/state';
import { Plus, X, Maximize2, Minimize2 } from 'lucide-react';

type TabType = 'terminal' | 'output' | 'problems';

export function TerminalPanel() {
  const terminalRef = useRef<HTMLDivElement>(null);
  const xtermRef = useRef<XTerm | null>(null);
  const fitAddonRef = useRef<FitAddon | null>(null);
  const sessionIdRef = useRef<string | null>(null);
  const [activeTab, setActiveTab] = useState<TabType>('terminal');
  const [terminalHeight, setTerminalHeight] = useState(200);

  const terminalPosition = useUIStore((s) => s.terminalPosition);

  useEffect(() => {
    if (!terminalRef.current || xtermRef.current) return;

    const xterm = new XTerm({
      fontSize: 12,
      fontFamily: "'JetBrains Mono', 'Fira Code', 'Consolas', monospace",
      fontWeight: '400',
      fontWeightBold: '600',
      lineHeight: 1.4,
      letterSpacing: 0,
      theme: {
        background: '#0d0d12',
        foreground: '#e4e4ef',
        cursor: '#4b7cf4',
        cursorAccent: '#0d0d12',
        selectionBackground: 'rgba(75, 124, 244, 0.25)',
        black: '#1a1a24',
        red: '#f87171',
        green: '#34d399',
        yellow: '#fbbf24',
        blue: '#4b7cf4',
        magenta: '#a78bfa',
        cyan: '#38bdf8',
        white: '#e4e4ef',
        brightBlack: '#555570',
        brightRed: '#f87171',
        brightGreen: '#34d399',
        brightYellow: '#fbbf24',
        brightBlue: '#4b7cf4',
        brightMagenta: '#a78bfa',
        brightCyan: '#38bdf8',
        brightWhite: '#ffffff',
      },
      cursorBlink: true,
      cursorStyle: 'bar',
      allowTransparency: true,
      scrollback: 5000,
      convertEol: true,
    });

    const fitAddon = new FitAddon();
    xterm.loadAddon(fitAddon);
    xterm.open(terminalRef.current);

    try { fitAddon.fit(); } catch { /* hidden */ }

    xtermRef.current = xterm;
    fitAddonRef.current = fitAddon;

    // Create PTY session
    invoke<string>('create_terminal').then((sid) => {
      sessionIdRef.current = sid;
      // Trigger initial resize based on fitted rows/cols
      try {
        fitAddon.fit();
        invoke('terminal_resize', {
          sessionId: sid,
          columns: xterm.cols,
          rows: xterm.rows,
        }).catch(() => undefined);
      } catch { /* hidden */ }
    }).catch(() => {
      xterm.write('\x1b[31mFailed to create terminal session. Backend may not be running.\x1b[0m\r\n');
    });

    // Listen for terminal output events
    const unlisten = listen<{ sessionId: string; data: string; stream: string }>('runtime:log', (event) => {
      if (event.payload.sessionId === sessionIdRef.current) {
        xterm.write(event.payload.data);
      }
    });

    xterm.onData((data) => {
      if (sessionIdRef.current) {
        invoke('terminal_write', { sessionId: sessionIdRef.current, data }).catch(() => undefined);
      }
    });

    xterm.onResize((size) => {
      if (sessionIdRef.current) {
        invoke('terminal_resize', {
          sessionId: sessionIdRef.current,
          columns: size.cols,
          rows: size.rows,
        }).catch(() => undefined);
      }
    });

    const observer = new ResizeObserver(() => {
      try {
        fitAddon.fit();
      } catch { /* hidden */ }
    });
    observer.observe(terminalRef.current);

    return () => {
      observer.disconnect();
      unlisten.then((fn) => { fn(); }).catch(() => undefined);
      if (sessionIdRef.current) {
        invoke('close_terminal', { sessionId: sessionIdRef.current }).catch(() => undefined);
      }
      xterm.dispose();
      xtermRef.current = null;
      fitAddonRef.current = null;
    };
  }, []);

  const handleSplitterDrag = useCallback(
    (e: React.MouseEvent) => {
      e.preventDefault();
      const startY = e.clientY;
      const startH = terminalHeight;

      const onMove = (ev: MouseEvent) => {
        const diff = startY - ev.clientY;
        const newH = Math.max(80, Math.min(600, startH - diff));
        setTerminalHeight(newH);
        requestAnimationFrame(() => {
          try { fitAddonRef.current?.fit(); } catch { /* hidden */ }
        });
      };
      const onUp = () => {
        document.removeEventListener('mousemove', onMove);
        document.removeEventListener('mouseup', onUp);
      };
      document.addEventListener('mousemove', onMove);
      document.addEventListener('mouseup', onUp);
    },
    [terminalHeight],
  );

  return (
    <div className="terminal-panel" style={terminalPosition === 'bottom' ? { height: terminalHeight } : { flex: 1 }}>
      <div className="terminal-header">
        <div className="terminal-tabs">
          {(['terminal', 'output', 'problems'] as TabType[]).map((tab) => (
            <button
              key={tab}
              className={`terminal-tab-btn ${activeTab === tab ? 'active' : ''}`}
              onClick={() => { setActiveTab(tab); }}
            >
              {tab.toUpperCase()}
            </button>
          ))}
        </div>
        <div className="terminal-toolbar">
          <button className="terminal-btn" title="New terminal">
            <Plus size={13} />
          </button>
          <button
            className="terminal-btn"
            title={terminalPosition === 'bottom' ? 'Dock right' : 'Dock bottom'}
            onClick={() => { useUIStore.getState().setTerminalPosition(terminalPosition === 'bottom' ? 'right' : 'bottom'); }}
          >
            {terminalPosition === 'bottom' ? <Maximize2 size={13} /> : <Minimize2 size={13} />}
          </button>
          <button
            className="terminal-btn"
            title="Close terminal"
            onClick={() => { useUIStore.getState().setTerminalPosition('collapsed'); }}
          >
            <X size={13} />
          </button>
        </div>
      </div>

      {terminalPosition === 'bottom' && (
        <div className="terminal-splitter" onMouseDown={handleSplitterDrag} />
      )}

      <div className="terminal-body" style={{ display: activeTab === 'terminal' ? 'block' : 'none' }}>
        <div ref={terminalRef} style={{ height: '100%', padding: '4px 0' }} />
      </div>

      {activeTab === 'output' && (
        <div className="terminal-body" style={{ padding: 16 }}>
          <div style={{ color: 'var(--text-muted)', fontSize: 12 }}>
            Task output will appear here when tasks are executed.
          </div>
        </div>
      )}

      {activeTab === 'problems' && (
        <div className="terminal-body" style={{ padding: 16 }}>
          <div style={{ color: 'var(--text-muted)', fontSize: 12 }}>
            No problems detected.
          </div>
        </div>
      )}
    </div>
  );
}