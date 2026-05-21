import { useState, useEffect, useRef } from 'react';
import { useWorkspaceStore } from '@local-flow/state';
import { EditorTabs } from './EditorTabs';
import { FileText, Save, Loader2, AlertTriangle } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';

// Global cache to preserve unsaved changes across tab switching
const unsavedChangesCache = new Map<string, { content: string; originalContent: string }>();

export function MainArea() {
  const activeTabId = useWorkspaceStore((s) => s.activeTabId);
  const openTabs = useWorkspaceStore((s) => s.openTabs);

  // Keep cache clean from closed tabs
  useEffect(() => {
    const openTabIds = new Set(openTabs.map((t) => t.id));
    for (const tabId of Array.from(unsavedChangesCache.keys())) {
      if (!openTabIds.has(tabId)) {
        unsavedChangesCache.delete(tabId);
      }
    }
  }, [openTabs]);

  return (
    <div className="main-area">
      {openTabs.length > 0 && <EditorTabs />}
      <div className="editor-content">
        {activeTabId ? (
          <FileViewer tabId={activeTabId} />
        ) : (
          <div className="editor-empty">
            <FileText size={48} className="editor-empty-icon" />
            <h2>LocalFlow IDE</h2>
            <p>
              Open a file from the explorer to start editing.<br />
              The terminal runs your tasks in real time.
            </p>
            <div className="editor-empty-hint">
              <kbd>Ctrl</kbd>+<kbd>K</kbd>
              <span>Command palette</span>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

function FileViewer({ tabId }: { tabId: string }) {
  const openTabs = useWorkspaceStore((s) => s.openTabs);
  const markTabDirty = useWorkspaceStore((s) => s.markTabDirty);
  const updateCursorPosition = useWorkspaceStore((s) => s.updateCursorPosition);
  const activeTab = openTabs.find((t) => t.id === tabId);

  const [content, setContent] = useState<string>('');
  const [originalContent, setOriginalContent] = useState<string>('');
  const [loading, setLoading] = useState<boolean>(false);
  const [saving, setSaving] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const lineNumbersRef = useRef<HTMLDivElement>(null);

  // Synchronize scrolls
  const handleScroll = () => {
    if (textareaRef.current && lineNumbersRef.current) {
      lineNumbersRef.current.scrollTop = textareaRef.current.scrollTop;
    }
  };

  // Update line and column position in state
  const updateCursor = () => {
    const textarea = textareaRef.current;
    if (!textarea) return;

    const selectionStart = textarea.selectionStart;
    const textBeforeCursor = textarea.value.slice(0, selectionStart);
    const lines = textBeforeCursor.split('\n');
    const line = lines.length;
    const column = lines[lines.length - 1].length + 1;

    updateCursorPosition(tabId, line, column);
  };

  // Load file content or fetch from cache
  useEffect(() => {
    if (!activeTab) return;

    const cached = unsavedChangesCache.get(tabId);
    if (cached) {
      setContent(cached.content);
      setOriginalContent(cached.originalContent);
      setError(null);
      return;
    }

    const loadFile = async () => {
      setLoading(true);
      setError(null);
      try {
        const fileContent = await invoke<string>('read_file', { path: activeTab.filePath });
        setContent(fileContent);
        setOriginalContent(fileContent);
        unsavedChangesCache.set(tabId, { content: fileContent, originalContent: fileContent });
        markTabDirty(tabId, false);
      } catch (err: unknown) {
        console.error('Failed to read file:', err);
        setError(err instanceof Error ? err.message : String(err));
      } finally {
        setLoading(false);
      }
    };

    void loadFile();
  }, [tabId, activeTab?.filePath]);

  // Handle manual/auto change events
  const handleContentChange = (newVal: string) => {
    setContent(newVal);
    const isDirty = newVal !== originalContent;
    markTabDirty(tabId, isDirty);

    // Save current content in cache
    const cached = unsavedChangesCache.get(tabId);
    if (cached) {
      unsavedChangesCache.set(tabId, { ...cached, content: newVal });
    } else {
      unsavedChangesCache.set(tabId, { content: newVal, originalContent });
    }
  };

  // Save changes to disk
  const handleSave = async () => {
    if (!activeTab || saving) return;
    setSaving(true);
    setError(null);
    try {
      await invoke('write_file', { path: activeTab.filePath, content });
      setOriginalContent(content);
      markTabDirty(tabId, false);
      unsavedChangesCache.set(tabId, { content, originalContent: content });
    } catch (err: unknown) {
      console.error('Failed to write file:', err);
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setSaving(false);
    }
  };

  // Wire Ctrl+S / Cmd+S keystrokes
  useEffect(() => {
    const handleGlobalKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === 's') {
        e.preventDefault();
        void handleSave();
      }
    };
    window.addEventListener('keydown', handleGlobalKeyDown);
    return () => {
      window.removeEventListener('keydown', handleGlobalKeyDown);
    };
  }, [content, originalContent, activeTab, saving]);

  // Keyboard enhancements (e.g. Tab inserts spaces)
  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === 'Tab') {
      e.preventDefault();
      const textarea = e.currentTarget;
      const start = textarea.selectionStart;
      const end = textarea.selectionEnd;
      const value = textarea.value;

      const newValue = value.substring(0, start) + '    ' + value.substring(end);
      handleContentChange(newValue);

      // Restore selection and advance cursor by 4 spaces
      setTimeout(() => {
        textarea.selectionStart = textarea.selectionEnd = start + 4;
        updateCursor();
      }, 0);
    }
  };

  if (!activeTab) {
    return (
      <div className="editor-empty">
        <p>File not found</p>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="editor-loading">
        <Loader2 size={32} className="editor-loading-spinner" />
        <p>Loading {activeTab.fileName}...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="editor-error">
        <AlertTriangle size={32} className="editor-error-icon" />
        <p>Error loading file: {error}</p>
      </div>
    );
  }

  const lines = content.split('\n');

  return (
    <div className="editor-container">
      <div className="editor-header">
        <span style={{ fontWeight: 500 }}>{activeTab.fileName}</span>
        <span className="editor-header-path">{activeTab.filePath}</span>
        <div style={{ flex: 1 }} />
        <button
          className="editor-save-btn"
          onClick={() => { void handleSave(); }}
          disabled={saving || content === originalContent}
        >
          {saving ? (
            <Loader2 size={12} className="editor-loading-spinner" />
          ) : (
            <Save size={12} />
          )}
          <span>{saving ? 'Saving...' : 'Save'}</span>
        </button>
      </div>

      <div className="editor-layout">
        <div className="editor-line-numbers" ref={lineNumbersRef}>
          {lines.map((_, idx) => (
            <span key={idx} className="editor-line-number">
              {idx + 1}
            </span>
          ))}
        </div>
        <textarea
          ref={textareaRef}
          className="editor-textarea"
          value={content}
          onChange={(e) => { handleContentChange(e.target.value); }}
          onScroll={handleScroll}
          onKeyDown={handleKeyDown}
          onKeyUp={updateCursor}
          onMouseUp={updateCursor}
          onFocus={updateCursor}
          spellCheck={false}
          autoFocus
        />
      </div>
    </div>
  );
}