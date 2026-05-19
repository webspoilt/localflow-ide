import { useEffect } from 'react';
import { useRuntimeStore, useTerminalStore } from '@zynta/state';
import type { RuntimeAction } from '@zynta/shared-types';
import { rootLogger } from '@zynta/logging';

const logger = rootLogger.child('runtime-events');

export function useRuntimeEvents() {
  const dispatch = useRuntimeStore((s) => s.dispatch);
  const appendOutput = useTerminalStore((s) => s.appendOutput);

  useEffect(() => {
    let unlisten: Array<() => void> = [];

    async function setup() {
      try {
        const { listen } = await import('@tauri-apps/api/event');

        const unlistenTaskEvent = await listen<RuntimeAction>('runtime:event', (event) => {
          dispatch(event.payload);
        });
        unlisten.push(unlistenTaskEvent);

        const unlistenLog = await listen('runtime:log', (event) => {
          const payload = event.payload as { sessionId: string; data: string; stream: string; timestamp: number };
          appendOutput({
            sessionId: payload.sessionId,
            data: payload.data,
            stream: payload.stream as 'stdout' | 'stderr',
            timestamp: payload.timestamp ?? Date.now(),
          });
        });
        unlisten.push(unlistenLog);

        logger.info('runtime_connected', 'Connected to runtime event bus');
      } catch (err) {
        logger.warn('runtime_connection_failed', 'Runtime event bus not available (Tauri backend not running)', {
          error: String(err),
        });
      }
    }

    setup();

    return () => {
      unlisten.forEach((fn) => fn());
    };
  }, [dispatch, appendOutput]);
}
