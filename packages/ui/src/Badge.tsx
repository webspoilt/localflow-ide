import type { ReactNode } from 'react';

interface BadgeProps {
  children: ReactNode;
  variant?: 'default' | 'success' | 'warning' | 'error' | 'info';
}

const badgeColors = {
  default: { background: 'var(--bg-tertiary)', color: 'var(--text-secondary)' },
  success: { background: 'rgba(62, 191, 107, 0.15)', color: 'var(--accent-green)' },
  warning: { background: 'rgba(230, 184, 62, 0.15)', color: 'var(--accent-yellow)' },
  error: { background: 'rgba(231, 76, 76, 0.15)', color: 'var(--accent-red)' },
  info: { background: 'rgba(74, 140, 255, 0.15)', color: 'var(--accent-blue)' },
} as const;

export function Badge({ children, variant = 'default' }: BadgeProps) {
  return (
    <span
      style={{
        display: 'inline-flex',
        alignItems: 'center',
        padding: '1px 6px',
        borderRadius: 'var(--radius-sm)',
        fontSize: 11,
        fontWeight: 500,
        ...badgeColors[variant],
      }}
    >
      {children}
    </span>
  );
}
