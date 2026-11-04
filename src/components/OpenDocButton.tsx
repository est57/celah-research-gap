import { useState } from "react";
import { t } from "../i18n";
import { isDesktop } from "../lib/api";
import { openDocument, type DocLinks } from "../lib/apiOa";

/** "PDF", "Open access" or "DOI" for a document; opens it in the default browser. */
export function OpenDocButton({ links }: { links: DocLinks | undefined }) {
  const [error, setError] = useState<string | null>(null);
  if (!links || !(links.open || links.doi)) return null;
  const label = links.pdf ? t("PDF") : links.open ? t("Akses terbuka") : t("DOI");
  const open = () => {
    if (!isDesktop()) return;
    setError(null);
    openDocument(links.id).catch((e: unknown) => setError(e instanceof Error ? e.message : String(e)));
  };
  return (
    <>
      <button type="button" className="btn doc-open" data-open={links.open} onClick={open}
        title={links.open ? t("Buka salinan akses terbuka di browser") : t("Buka halaman penerbit lewat DOI")}>
        {label}
      </button>
      {error ? <span className="refs-error" role="alert">{error}</span> : null}
    </>
  );
}
