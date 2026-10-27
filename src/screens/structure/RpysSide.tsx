import { useEffect, useState, type FormEvent } from "react";
import { formatInt } from "../../lib/format";
import { DEFAULT_RPYS, type Rpys, type RpysParams } from "../../lib/rpysTypes";

interface Props {
  params: RpysParams;
  data: Rpys | null;
  busy: boolean;
  onParams: (p: RpysParams) => void;
}

/** "" is automatic (0); otherwise a whole year 1800 to 2100, or null when invalid. */
function year(text: string): number | null {
  if (text.trim() === "") return 0;
  const v = Number(text);
  return Number.isInteger(v) && v >= 1800 && v <= 2100 ? v : null;
}

export function RpysSide({ params, data, busy, onParams }: Props) {
  const [from, setFrom] = useState(params.from ? String(params.from) : "");
  const [to, setTo] = useState(params.to ? String(params.to) : "");
  const [error, setError] = useState<string | null>(null);
  useEffect(() => {
    setFrom(params.from ? String(params.from) : "");
    setTo(params.to ? String(params.to) : "");
  }, [params]);

  const submit = (e: FormEvent) => {
    e.preventDefault();
    const [a, b] = [year(from), year(to)];
    if (a === null || b === null || (a && b && a > b)) {
      setError("Isi tahun 1800 sampai 2100, tahun awal tidak setelah tahun akhir. Kosongkan untuk otomatis.");
      return;
    }
    setError(null);
    onParams({ from: a, to: b });
  };

  return (
    <aside className="inspector" aria-label="Parameter RPYS">
      <div className="ph">Parameter</div>
      <form className="params" onSubmit={submit}>
        <div className="param-row">
          <label htmlFor="rp-from">Dari tahun</label>
          <input id="rp-from" className="input num" inputMode="numeric" maxLength={4} value={from}
            placeholder={data ? String(data.from) : "otomatis"} onChange={(e) => setFrom(e.target.value)} />
        </div>
        <div className="param-row">
          <label htmlFor="rp-to">Sampai tahun</label>
          <input id="rp-to" className="input num" inputMode="numeric" maxLength={4} value={to}
            placeholder={data ? String(data.to) : "otomatis"} onChange={(e) => setTo(e.target.value)} />
        </div>
        {error ? <p className="evo-error" role="alert">{error}</p> : null}
        <div className="param-actions">
          <button type="button" className="btn" onClick={() => onParams(DEFAULT_RPYS)}>Atur ulang</button>
          <button type="submit" className="btn btn-primary" disabled={busy}>Terapkan</button>
        </div>
      </form>
      <div className="ph">Ringkasan</div>
      {data ? (
        <dl className="words-facts">
          <dt>Dokumen dengan rujukan teks</dt>
          <dd className="num">{`${formatInt(data.citing)} dari ${formatInt(data.documents)}`}</dd>
          <dt>Sitasi bertahun</dt>
          <dd className="num">{formatInt(data.dated)}</dd>
          <dt>Rujukan tanpa tahun</dt>
          <dd className="num">{formatInt(data.undated)}</dd>
        </dl>
      ) : null}
      <p className="muted words-hint">
        Tahun yang jauh di atas median lima tahun di sekitarnya menandai karya yang sering dirujuk bersama: akar sejarah
        bidang ini. Bandingkan dengan koleksi Anda: karya klasik yang belum dibaca layak masuk tinjauan pustaka. Rujukan
        dengan DOI dihitung sebagai satu karya walau ditulis dengan format berbeda.
      </p>
    </aside>
  );
}
