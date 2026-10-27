import type { ReactNode } from "react";

/** Centered message used for loading, empty and error states of analysis screens. */
export function ScreenMessage({ children }: { children: ReactNode }) {
  return (
    <div className="placeholder">
      <div>{children}</div>
    </div>
  );
}

export function EmptyCollection() {
  return (
    <ScreenMessage>
      <strong>Belum ada dokumen</strong>
      <span className="muted">Kumpulkan dokumen di Koleksi &amp; impor, lalu kembali ke sini.</span>
    </ScreenMessage>
  );
}
