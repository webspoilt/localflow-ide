import { useRef, useEffect, useCallback, useState } from 'react';
import { Terminal as XTerm } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import '@xterm/xterm/css/xterm.css';
import { invoke } from '@tauri-apps/api/core';
import { useUIStore } from '@zynta/state';
import { Plus, X, Maximize2, Minimize2, Terminal as TerminalIcon } from 'lucide-react';

type TabType = 'terminal' | 'output' | 'problems';

export function TerminalPanel() {
  const terminalRef = useRef<HTMLDivElement>(null);
  const xtermRef = useRef<XTerm | null>(null);
  const fitAddonRef = useRef<FitAddon | null>(null);
  const inputBufferRef = useRef('');
  const [activeTab, setActiveTab] = useState<TabType>('terminal');
  const [terminalHeight, setTerminalHeight] = useState(200);

  const terminalPosition = useUIStore((s) => s.terminalPosition);
  const setTerminalPosition = useUIStore((s) => s.setTerminalPosition);

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
      scrollback: 1000,
      convertEol: true,
    });

    const fitAddon = new FitAddon();
    xterm.loadAddon(fitAddon);
    xterm.open(terminalRef.current);

    try { fitAddon.fit(); } catch { /* container hidden */ }

    const prompt = '\r\n\x1b[90m$\x1b[0m ';
    xterm.write('\x1b[36mZynta Studio\x1b[0m \x1b[90mruntime\x1b[0m\r\n');
    xterm.write('Type commands. Press Enter to execute.\r\n');
    xterm.write(prompt);

    xterm.onData((data) => {
      const code = data.charCodeAt(0);

      if (code === 13) {
        const cmd = inputBufferRef.current.trim();
        inputBufferRef.current = '';

        if (cmd) {
          xterm.write('\r\n');
          executeCommand(cmd, xterm);
        }
        xterm.write(prompt);
        return;
      }

      if (code === 127) {
        if (inputBufferRef.current.length > 0) {
          inputBufferRef.current = inputBufferRef.current.slice(0, -1);
          xterm.write('\b \b');
        }
        return;
      }

      if (code === 3) {
        xterm.write('^C\r\n');
        inputBufferRef.current = '';
        xterm.write(prompt);
        return;
      }

      if (code < 32) return;

      inputBufferRef.current += data;
      xterm.write(data);
    });

    xtermRef.current = xterm;
    fitAddonRef.current = fitAddon;

    const observer = new ResizeObserver(() => {
      try { fitAddon.fit(); } catch { /* ignore */ }
    });
    if (terminalRef.current) observer.observe(terminalRef.current);

    return () => {
      observer.disconnect();
      xterm.dispose();
      xtermRef.current = null;
      fitAddonRef.current = null;
    };
  }, []);

  const executeCommand = useCallback(async (cmd: string, xterm: XTerm) => {
    xterm.write(`\x1b[90mExecuting: ${cmd}\x1b[0m\r\n`);

    try {
      const result = await invoke<{ exit_code: number; stdout: string; stderr: string }>('execute_command', { command: cmd });

      if (result.stdout) {
        xterm.write(result.stdout);
      }
      if (result.stderr) {
        xterm.write(`\x1b[31m${result.stderr}\x1b[0m`);
      }

      if (result.exit_code === 0) {
        xterm.write(`\r\n\x1b[32m[Done]\x1b[0m`);
      } else {
        xterm.write(`\r\n\x1b[31m[Exit ${result.exit_code}]\x1b[0m`);
      }
    } catch (err) {
      xterm.write(`\x1b[31mError: ${String(err)}\x1b[0m`);
    }

    xterm.write('\r\n');
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
          try { fitAddonRef.current?.fit(); } catch { /* ignore */ }
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
              onClick={() => setActiveTab(tab)}
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
            onClick={() => setTerminalPosition(terminalPosition === 'bottom' ? 'right' : 'bottom')}
          >
            {terminalPosition === 'bottom' ? <Maximize2 size={13} /> : <Minimize2 size={13} />}
          </button>
          <button
            className="terminal-btn"
            title="Close terminal"
            onClick={() => setTerminalPosition('collapsed')}
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