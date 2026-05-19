import { useRef, useEffect, useCallback } from 'react';
import { Terminal as XTerm } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import '@xterm/xterm/css/xterm.css';
import { useUIStore, useTerminalStore } from '@zynta/state';
import { Plus, X, Maximize2, Minimize2 } from 'lucide-react';

export function TerminalPanel() {
  const terminalRef = useRef<HTMLDivElement>(null);
  const xtermRef = useRef<XTerm | null>(null);
  const fitAddonRef = useRef<FitAddon | null>(null);
  const terminalPosition = useUIStore((s) => s.terminalPosition);
  const setTerminalPosition = useUIStore((s) => s.setTerminalPosition);
  const setTerminalHeight = useUIStore((s) => s.setTerminalHeight);
  const terminalHeight = useUIStore((s) => s.terminalHeight);
  const sessions = useTerminalStore((s) => s.sessions);
  const activeSessionId = useTerminalStore((s) => s.activeSessionId);
  const setActiveSession = useTerminalStore((s) => s.setActiveSession);
  const appendOutput = useTerminalStore((s) => s.appendOutput);

  useEffect(() => {
    if (!terminalRef.current || xtermRef.current) return;

    const xterm = new XTerm({
      fontSize: 12,
      fontFamily: "'JetBrains Mono', 'Fira Code', 'Consolas', monospace",
      theme: {
        background: '#12121a',
        foreground: '#e2e2f0',
        cursor: '#4a8cff',
        selectionBackground: 'rgba(74, 140, 255, 0.3)',
        black: '#1a1a26',
        red: '#e74c4c',
        green: '#3ebf6b',
        yellow: '#e6b83e',
        blue: '#4a8cff',
        magenta: '#8b5cf6',
        cyan: '#38bdf8',
        white: '#e2e2f0',
        brightBlack: '#6a6a88',
        brightRed: '#e74c4c',
        brightGreen: '#3ebf6b',
        brightYellow: '#e6b83e',
        brightBlue: '#4a8cff',
        brightMagenta: '#8b5cf6',
        brightCyan: '#38bdf8',
        brightWhite: '#ffffff',
      },
      allowTransparency: true,
      cursorBlink: true,
      cols: 80,
      rows: 15,
    });

    const fitAddon = new FitAddon();
    xterm.loadAddon(fitAddon);
    xterm.open(terminalRef.current);
    fitAddon.fit();

    xterm.write('Welcome to Zynta Studio runtime terminal\r\n');
    xterm.write('Type commands or wait for runtime tasks\r\n');
    xterm.write('$ ');

    xterm.onData((data) => {
      xterm.write(data);
    });

    xtermRef.current = xterm;
    fitAddonRef.current = fitAddon;

    const resizeFn = () => {
      try {
        fitAddon.fit();
      } catch {
        // container might be hidden during resize
      }
    };

    const observer = new ResizeObserver(resizeFn);
    observer.observe(terminalRef.current);

    return () => {
      observer.disconnect();
      xterm.dispose();
      xtermRef.current = null;
      fitAddonRef.current = null;
    };
  }, []);

  const handleResizeStart = useCallback(
    (e: React.MouseEvent) => {
      e.preventDefault();
      const startY = e.clientY;
      const startHeight = terminalHeight;

      const onMove = (ev: MouseEvent) => {
        const diff = startY - ev.clientY;
        const newHeight = Math.max(100, Math.min(500, startHeight - diff));
        setTerminalHeight(newHeight);
        requestAnimationFrame(() => {
          fitAddonRef.current?.fit();
        });
      };

      const onUp = () => {
        document.removeEventListener('mousemove', onMove);
        document.removeEventListener('mouseup', onUp);
      };

      document.addEventListener('mousemove', onMove);
      document.addEventListener('mouseup', onUp);
    },
    [terminalHeight, setTerminalHeight],
  );

  return (
    <div
      className="terminal-panel"
      style={terminalPosition === 'bottom' ? { height: terminalHeight } : undefined}
    >
      <div className="terminal-header">
        <div className="terminal-tabs">
          <button className="terminal-tab-btn active">TERMINAL</button>
          <button className="terminal-tab-btn">OUTPUT</button>
          <button className="terminal-tab-btn">PROBLEMS</button>
        </div>
        <div className="terminal-toolbar">
          <button className="terminal-toolbar-btn" title="New terminal">
            <Plus size={14} />
          </button>
          <button
            className="terminal-toolbar-btn"
            title={terminalPosition === 'bottom' ? 'Move to right' : 'Move to bottom'}
            onClick={() => setTerminalPosition(terminalPosition === 'bottom' ? 'right' : 'bottom')}
          >
            {terminalPosition === 'bottom' ? <Maximize2 size={12} /> : <Minimize2 size={12} />}
          </button>
          <button
            className="terminal-toolbar-btn"
            title="Close terminal"
            onClick={() => setTerminalPosition('collapsed')}
          >
            <X size={14} />
          </button>
        </div>
      </div>
      {terminalPosition === 'bottom' && (
        <div className="splitter splitter-horizontal" onMouseDown={handleResizeStart} />
      )}
      <div ref={terminalRef} className="terminal-body" style={{ padding: 0 }} />
    </div>
  );
}
