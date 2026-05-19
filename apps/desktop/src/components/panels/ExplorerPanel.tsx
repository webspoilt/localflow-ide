import { useCallback, useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { useWorkspaceStore, useUIStore } from '@zynta/state';
import type { FileNode } from '@zynta/state';
import {
  FileText,
  Folder,
  FolderOpen,
  ChevronRight,
  RefreshCw,
  Plus,
  Search,
  Command,
} from 'lucide-react';

const FILE_ICONS: Record<string, string> = {
  ts: '📘',
  tsx: '📘',
  js: '📒',
  jsx: '📒',
  json: '📋',
  rs: '🦀',
  css: '🎨',
  html: '🌐',
  md: '📝',
  py: '🐍',
  go: '🔵',
  toml: '⚙️',
  yaml: '⚙️',
  yml: '⚙️',
  sh: '🖥️',
  default: '📄',
};

function getFileEmoji(name: string): string {
  const ext = name.split('.').pop() ?? '';
  return FILE_ICONS[ext] ?? FILE_ICONS.default;
}

export function ExplorerPanel() {
  const root = useWorkspaceStore((s) => s.root);
  const openFile = useWorkspaceStore((s) => s.openFile);
  const setRoot = useWorkspaceStore((s) => s.setRoot);
  const selectedFilePath = useWorkspaceStore((s) => s.selectedFilePath);
  const selectFile = useWorkspaceStore((s) => s.selectFile);
  const [loading, setLoading] = useState(false);

  const handleOpenFolder = useCallback(async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: 'Open Workspace Folder',
      });
      if (selected) {
        setLoading(true);
        try {
          const files = await invoke<string>('read_directory', { path: selected });
          const parsed = JSON.parse(files) as FileNode;
          setRoot(parsed);
        } catch {
          const mockRoot: FileNode = {
            id: crypto.randomUUID(),
            name: (selected as string).split(/[\\/]/).pop() ?? 'workspace',
            path: selected as string,
            type: 'directory',
            children: [
              {
                id: crypto.randomUUID(),
                name: 'src',
                path: `${selected}/src`,
                type: 'directory',
                children: [
                  {
                    id: crypto.randomUUID(),
                    name: 'index.ts',
                    path: `${selected}/src/index.ts`,
                    type: 'file',
                    language: 'typescript',
                  },
                  {
                    id: crypto.randomUUID(),
                    name: 'app.tsx',
                    path: `${selected}/src/app.tsx`,
                    type: 'file',
                    language: 'typescript',
                  },
                ],
              },
              {
                id: crypto.randomUUID(),
                name: 'README.md',
                path: `${selected}/README.md`,
                type: 'file',
                language: 'markdown',
              },
              {
                id: crypto.randomUUID(),
                name: 'package.json',
                path: `${selected}/package.json`,
                type: 'file',
                language: 'json',
              },
            ],
          };
          setRoot(mockRoot);
        } finally {
          setLoading(false);
        }
      }
    } catch (err) {
      console.error('Failed to open folder:', err);
      setLoading(false);
    }
  }, [setRoot]);

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
      <div style={{ display: 'flex', gap: 2, padding: '4px 8px', borderBottom: '1px solid var(--border-color)' }}>
        <button
          className="terminal-btn"
          onClick={handleOpenFolder}
          title="Open Folder"
          style={{ display: 'flex', alignItems: 'center', gap: 4, width: '100%', justifyContent: 'flex-start', padding: '4px 6px' }}
        >
          <FolderOpen size={13} />
          <span style={{ fontSize: 11 }}>Open Folder</span>
        </button>
      </div>

      {!root ? (
        <div className="empty-state" style={{ flex: 1, gap: 8 }}>
          <FolderOpen size={32} className="empty-state-icon" />
          <p style={{ fontSize: 12, color: 'var(--text-muted)' }}>
            No folder open
          </p>
          <button
            onClick={handleOpenFolder}
            style={{
              padding: '6px 12px',
              background: 'var(--accent-blue)',
              color: 'white',
              border: 'none',
              borderRadius: 'var(--radius-sm)',
              fontSize: 12,
              cursor: 'pointer',
            }}
          >
            Open Folder
          </button>
        </div>
      ) : (
        <div className="file-tree" style={{ flex: 1, overflow: 'auto', padding: '4px 0' }}>
          <FileTreeNode
            node={root}
            depth={0}
            onSelect={(node) => {
              if (node.type === 'directory') {
                node.children ??= [];
              } else {
                openFile(node);
              }
            }}
            selectedPath={selectedFilePath}
            onHover={(path) => selectFile(path)}
          />
        </div>
      )}

      <div style={{
        padding: '6px 12px',
        borderTop: '1px solid var(--border-color)',
        display: 'flex',
        gap: 6,
      }}>
        <button className="terminal-btn" title="New File" onClick={() => {}}>
          <Plus size={14} />
        </button>
        <button className="terminal-btn" title="Refresh" onClick={handleOpenFolder}>
          <RefreshCw size={14} />
        </button>
        <button className="terminal-btn" title="Search" onClick={() => useUIStore.getState().setActivePanel('search')}>
          <Search size={14} />
        </button>
        <div style={{ flex: 1 }} />
        <button className="terminal-btn" title="Command Palette" onClick={() => {}}>
          <Command size={14} />
        </button>
      </div>
    </div>
  );
}

function FileTreeNode({
  node,
  depth,
  onSelect,
  selectedPath,
  onHover,
}: {
  node: FileNode;
  depth: number;
  onSelect: (node: FileNode) => void;
  selectedPath: string | null;
  onHover: (path: string) => void;
}) {
  const [expanded, setExpanded] = useState(depth < 1);
  const isDir = node.type === 'directory';
  const isSelected = selectedPath === node.path;

  const paddingLeft = 8 + depth * 16;

  return (
    <>
      <div
        className={`file-item ${isSelected ? 'selected' : ''}`}
        style={{ paddingLeft }}
        onClick={() => {
          if (isDir) {
            setExpanded(!expanded);
          } else {
            onSelect(node);
          }
        }}
        onMouseEnter={() => onHover(node.path)}
      >
        {isDir && (
          <ChevronRight
            size={12}
            className={`file-item-chevron ${expanded ? 'expanded' : ''}`}
          />
        )}
        <span className="file-item-icon">
          {isDir ? (
            expanded ? <FolderOpen size={14} /> : <Folder size={14} />
          ) : (
            <FileText size={14} />
          )}
        </span>
        <span className="file-item-name">{node.name}</span>
      </div>

      {isDir && expanded && node.children?.map((child) => (
        <FileTreeNode
          key={child.path}
          node={child}
          depth={depth + 1}
          onSelect={onSelect}
          selectedPath={selectedPath}
          onHover={onHover}
        />
      ))}
    </>
  );
}