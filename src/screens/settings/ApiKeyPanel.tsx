import { useState, type FormEvent, type ReactNode } from "react";
import type { ApiKeyState } from "../../hooks/useApiKey";
import { t } from "../../i18n";

const KEY_PATTERN = /^[A-Za-z0-9_-]{8,128}$/;

interface Props {
  api: ApiKeyState;
  /** Prefix for element ids, unique per panel. */
  id: string;
  title: string;
  /** API host the key is sent to, shown to the user. */
  host: string;
  /** Why the key helps and where to get it. */
  intro: ReactNode;
}

export function ApiKeyPanel({ api, id, title, host, intro }: Props) {
  const [value, setValue] = useState("");
  const [confirmRemove, setConfirmRemove] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  const { status, error, busy } = api;

  const submit = async (e: FormEvent) => {
    e.preventDefault();
    const key = value.trim();
    if (!KEY_PATTERN.test(key)) {
      setNotice(t("Kunci harus 8 sampai 128 karakter berisi huruf, angka, '-' atau '_'."));
      return;
    }
    setNotice(null);
    if (await api.save(key)) {
      setValue("");
      setNotice(t("Kunci API tersimpan di keychain sistem."));
    }
  };

  const remove = async () => {
    setConfirmRemove(false);
    if (await api.remove()) setNotice(t("Kunci API dihapus dari keychain."));
  };

  return (
    <section className="settings-card" aria-labelledby={`${id}-title`}>
      <div className="ph" id={`${id}-title`}>
        {title}
        <span className="key-state" data-on={status.configured}>
          {status.configured ? t("Tersimpan {hint}", { hint: status.hint ?? "" }) : t("Belum diatur")}
        </span>
      </div>
      <div className="settings-body">
        <p className="muted">{intro}</p>
        <form className="key-form" onSubmit={submit}>
          <label className="field field-grow" htmlFor={`${id}-key`}>
            {status.configured ? t("Ganti kunci") : t("Kunci API")}
            <input
              id={`${id}-key`}
              className="input num"
              type="password"
              autoComplete="off"
              spellCheck={false}
              maxLength={128}
              value={value}
              placeholder={t("Tempel kunci API")}
              onChange={(e) => setValue(e.target.value)}
            />
          </label>
          <button type="submit" className="btn btn-primary" disabled={busy || value.trim() === ""}>
            {t("Simpan")}
          </button>
        </form>
        {status.configured && !confirmRemove ? (
          <button type="button" className="btn" disabled={busy} onClick={() => setConfirmRemove(true)}>
            {t("Hapus kunci")}
          </button>
        ) : null}
        {confirmRemove ? (
          <div className="notice confirm-row" role="alertdialog" aria-label={t("Konfirmasi hapus kunci")}>
            <span>{t("Hapus kunci dari keychain? Pencarian akan kembali memakai kuota bersama.")}</span>
            <button type="button" className="btn" onClick={() => setConfirmRemove(false)}>{t("Batal")}</button>
            <button type="button" className="btn btn-danger" disabled={busy} onClick={() => void remove()}>
              {t("Hapus")}
            </button>
          </div>
        ) : null}
        {error ? <p className="message" data-kind="error" role="alert">{error}</p> : null}
        {notice && !error ? <p className="message" role="status">{notice}</p> : null}
        <ul className="muted security-notes">
          <li>{t("Kunci disimpan di keychain sistem operasi, bukan di file atau database aplikasi.")}</li>
          <li>{t("Tampilan aplikasi hanya menerima empat karakter terakhir kunci.")}</li>
          <li>{t("Kunci hanya dikirim ke {host} melalui HTTPS.", { host })}</li>
        </ul>
      </div>
    </section>
  );
}
