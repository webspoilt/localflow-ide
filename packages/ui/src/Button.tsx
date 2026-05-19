import type { ButtonHTMLAttributes, ReactNode } from 'react';

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'ghost' | 'danger';
  size?: 'sm' | 'md' | 'lg';
  children: ReactNode;
}

const variantStyles = {
  primary: {
    background: 'var(--accent-blue)',
    color: 'white',
    border: 'none',
  },
  secondary: {
    background: 'var(--bg-tertiary)',
    color: 'var(--text-primary)',
    border: '1px solid var(--border-color)',
  },
  ghost: {
    background: 'transparent',
    color: 'var(--text-secondary)',
    border: 'none',
  },
  danger: {
    background: 'var(--accent-red)',
    color: 'white',
    border: 'none',
  },
} as const;

const sizeStyles = {
  sm: { padding: '4px 8px', fontSize: 11 },
  md: { padding: '6px 12px', fontSize: 13 },
  lg: { padding: '8px 16px', fontSize: 14 },
} as const;

export function Button({ variant = 'secondary', size = 'md', children, style, ...props }: ButtonProps) {
  return (
    <button
      style={{
        borderRadius: 'var(--radius-md)',
        cursor: 'pointer',
        display: 'inline-flex',
        alignItems: 'center',
        gap: 6,
        fontWeight: 500,
        transition: 'opacity var(--transition-fast)',
        ...variantStyles[variant],
        ...sizeStyles[size],
        ...style,
      }}
      onMouseEnter={(e) => {
        e.currentTarget.style.opacity = '0.85';
      }}
      onMouseLeave={(e) => {
        e.currentTarget.style.opacity = '1';
      }}
      {...props}
    >
      {children}
    </button>
  );
}
