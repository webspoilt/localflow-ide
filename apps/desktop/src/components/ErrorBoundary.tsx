import { Component, type ReactNode, type ErrorInfo } from 'react';

interface Props { children: ReactNode; }

interface State {
  hasError: boolean;
  error: Error | null;
}

export class ErrorBoundary extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = { hasError: false, error: null };
  }

  static getDerivedStateFromError(error: Error): Partial<State> {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo): void {
    console.error('[ErrorBoundary]', error, errorInfo);
  }

  handleReset = (): void => {
    this.setState({ hasError: false, error: null });
  };

  render(): ReactNode {
    if (this.state.hasError) {
      return (
        <div className="error-boundary">
          <h3>Something went wrong</h3>
          <p style={{ color: 'var(--text-secondary)', fontSize: 12 }}>
            The application encountered an error and may be in an inconsistent state.
          </p>
          <button onClick={this.handleReset}>
            Reset
          </button>
          {this.state.error && (
            <pre>
              {this.state.error.message}
              {'\n'}
              {this.state.error.stack}
            </pre>
          )}
        </div>
      );
    }
    return this.props.children;
  }
}