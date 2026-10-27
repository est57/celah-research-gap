import { useCallback, useEffect, useState } from "react";
import { apiKeyStatus, clearApiKey, isDesktop, setApiKey } from "../lib/api";
import type { KeyStatus, ServiceId } from "../lib/types";

const NONE: KeyStatus = { configured: false, hint: null };

export function useApiKey(service: ServiceId) {
  const [status, setStatus] = useState<KeyStatus>(NONE);
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  const run = useCallback(async (action: () => Promise<KeyStatus>) => {
    setBusy(true);
    try {
      setStatus(await action());
      setError(null);
      return true;
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
      return false;
    } finally {
      setBusy(false);
    }
  }, []);

  useEffect(() => {
    if (isDesktop()) void run(() => apiKeyStatus(service));
  }, [run, service]);

  return {
    status,
    error,
    busy,
    save: (key: string) => run(() => setApiKey(service, key)),
    remove: () => run(() => clearApiKey(service)),
  };
}

export type ApiKeyState = ReturnType<typeof useApiKey>;
