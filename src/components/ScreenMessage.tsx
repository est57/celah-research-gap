import type { ReactNode } from "react";
import { t } from "../i18n";

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
      <strong>{t("Belum ada dokumen")}</strong>
      <span className="muted">{t("Kumpulkan dokumen di Koleksi & impor, lalu kembali ke sini.")}</span>
    </ScreenMessage>
  );
}
