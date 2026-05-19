import { useWorkspaceStore } from '@zynta/state';
import { File, Folder, FolderOpen, ChevronRight } from 'lucide-react';
import { useState } from 'react';
import type { FileNode } from '@zynta/state';

export function ExplorerPanel() {
  const root = useWorkspaceStore((s) => s.root);
  const openFile = useWorkspaceStore((s) => s.openFile);

  if (!root) {
    return (
      <div style={{ padding: 16, color: 'var(--text-muted)', textAlign: 'center' }}>
        <p>No workspace open</p>
        <p style={{ fontSize: 11, marginTop: 4 }}>Open a folder to explore files</p>
      </div>
    );
  }

  return (
    <div className="fade-in">
      <FileTreeNode node={root} depth={0} onSelect={openFile} />
    </div>
  );
}

function FileTreeNode({
  node,
  depth,
  onSelect,
}: {
  node: FileNode;
  depth: number;
  onSelect: (node: FileNode) => void;
}) {
  const [expanded, setExpanded] = useState(depth < 1);
  const isDir = node.type === 'directory';
  const selectedFilePath = useWorkspaceStore((s) => s.selectedFilePath);

  const handleClick = () => {
    if (isDir) {
      setExpanded(!expanded);
    } else {
      onSelect(node);
    }
  };

  return (
    <>
      <div
        className={`file-tree-item ${selectedFilePath === node.path ? 'selected' : ''}`}
        style={{ paddingLeft: 8 + depth * 16 }}
        onClick={handleClick}
        title={node.path}
      >
        {isDir && (
          <ChevronRight size={12} className={`chevron ${expanded ? 'open' : ''}`} style={{ marginRight: 2 }} />
        )}
        {isDir ? (
          expanded ? (
            <FolderOpen size={14} className="icon" />
          ) : (
            <Folder size={14} className="icon" />
          )
        ) : (
          <File size={14} className="icon" />
        )}
        <span>{node.name}</span>
      </div>
      {isDir && expanded && node.children?.map((child: FileNode) => (
        <FileTreeNode key={child.path} node={child} depth={depth + 1} onSelect={onSelect} />
      ))}
    </>
  );
}
