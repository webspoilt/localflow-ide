import { Component, type ErrorInfo, type ReactNode } from 'react';

interface Props {
  children: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
  errorInfo: ErrorInfo | null;
}

export class ErrorBoundary extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = { hasError: false, error: null, errorInfo: null };
  }

  static getDerivedStateFromError(error: Error): Partial<State> {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo): void {
    this.setState({ errorInfo });
    console.error('[ErrorBoundary]', error, errorInfo);
  }

  handleReset = () => {
    this.setState({ hasError: false, error: null, errorInfo: null });
  };

  render() {
    if (this.state.hasError) {
      return (
        <div className="error-boundary">
          <h3>Application Error</h3>
          <p style={{ color: 'var(--text-secondary)' }}>
            Something went wrong. The runtime may be in an inconsistent state.
          </p>
          <button
            onClick={this.handleReset}
            style={{
              marginTop: 16,
              padding: '8px 16px',
              background: 'var(--accent-blue)',
              color: 'white',
              border: 'none',
              borderRadius: 'var(--radius-md)',
              cursor: 'pointer',
            }}
          >
            Reset Application
          </button>
          {this.state.error && (
            <pre>{this.state.error.message}{this.state.error.stack ? '\n' + this.state.error.stack : ''}</pre>
          )}
        </div>
      );
    }

    return this.props.children;
  }
}
