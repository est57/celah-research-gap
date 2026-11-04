import { useEffect, useState, type FormEvent } from "react";
import { useApiKey } from "../../hooks/useApiKey";
import { t } from "../../i18n";
import { isDesktop } from "../../lib/api";
import { findOpenAccess, openAccessStatus, type OaStatus } from "../../lib/apiOa";
import { formatInt } from "../../lib/format";

const EMAIL = /^[^\s@"<>,;]+@[^\s@"<>,;]+\.[^\s@"<>,;]+$/;

/** Contact email for Unpaywall, and the lookup of open-access PDFs for the collection. */
export function UnpaywallPanel() {
  const api = useApiKey("unpaywall");
  const [value, setValue] = useState("");
  const [note, setNote] = useState<string | null>(null);
  const [oa, setOa] = useState<OaStatus | null>(null);
  const [busy, setBusy] = useState(false);
  const refresh = () => void openAccessStatus().then(setOa).catch(() => setOa(null));
  useEffect(() => {
    if (isDesktop()) refresh();
  }, []);

  const submit = async (e: FormEvent) => {
    e.preventDefault();
    if (!EMAIL.test(value.trim())) return setNote(t("Isi alamat email yang valid."));
    setNote(null);
    if (await api.save(value.trim())) setValue("");
  };
  const run = async () => {
    setBusy(true);
    setNote(null);
    try {
      const r = await findOpenAccess();
      setNote(r.error ?? t("{checked} dokumen diperiksa, {found} punya salinan akses terbuka. Sisa {remaining}.", { checked: formatInt(r.checked), found: formatInt(r.found), remaining: formatInt(r.remaining) }));
      refresh();
    } catch (err) {
      setNote(err instanceof Error ? err.message : String(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <section className="settings-card" aria-labelledby="unpaywall-title">
      <div className="ph" id="unpaywall-title">
        {t("PDF akses terbuka (Unpaywall)")}
        <span className="key-state" data-on={api.status.configured}>
          {api.status.configured ? t("Tersimpan {hint}", { hint: api.status.hint ?? "" }) : t("Belum diatur")}
        </span>
      </div>
      <div className="settings-body">
        <p className="muted">
          {t("Unpaywall mencari salinan legal dan gratis dari dokumen ber-DOI. Unpaywall meminta alamat email Anda di setiap permintaan sebagai kontak; email hanya dikirim ke api.unpaywall.org dan disimpan di keychain sistem.")}
        </p>
        <form className="key-form" onSubmit={submit}>
          <label className="field field-grow" htmlFor="unpaywall-email">
            {api.status.configured ? t("Ganti email") : t("Email")}
            <input id="unpaywall-email" className="input" type="email" autoComplete="email" maxLength={254} value={value}
              placeholder={t("nama@kampus.ac.id")} onChange={(e) => setValue(e.target.value)} />
          </label>
          <button type="submit" className="btn btn-primary" disabled={api.busy || value.trim() === ""}>{t("Simpan")}</button>
          {api.status.configured ? <button type="button" className="btn" disabled={api.busy} onClick={() => void api.remove()}>{t("Hapus email")}</button> : null}
        </form>
        {oa ? (
          <p className="muted">
            {t("{open} dari {checked} dokumen yang sudah diperiksa punya salinan akses terbuka; {left} dari {doi} dokumen ber-DOI belum diperiksa.", {
              open: formatInt(oa.open), checked: formatInt(oa.checked), left: formatInt(oa.withDoi - oa.checked), doi: formatInt(oa.withDoi),
            })}
          </p>
        ) : null}
        <div className="about-actions">
          <button type="button" className="btn" disabled={busy || !api.status.configured || (oa !== null && oa.withDoi === oa.checked)} onClick={() => void run()}>
            {busy ? t("Mencari...") : t("Cari PDF akses terbuka (300 dokumen per kali)")}
          </button>
        </div>
        {api.error ? <p className="message" data-kind="error" role="alert">{api.error}</p> : null}
        {note ? <p className="message" role="status">{note}</p> : null}
      </div>
    </section>
  );
}
