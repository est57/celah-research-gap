import { formatInt } from "../../lib/format";
import { VERDICT_LABEL, type CheckSummary } from "../../lib/gapTypes";

/** SQLite stores "YYYY-MM-DD HH:MM:SS" in UTC; show it in the viewer's local time. */
function localTime(utc: string): string {
  const d = new Date(`${utc.replace(" ", "T")}Z`);
  return Number.isNaN(d.getTime()) ? utc : d.toLocaleString("id-ID", { dateStyle: "medium", timeStyle: "short" });
}

interface Props {
  check: CheckSummary;
  /** Papers listed; the rest are summarized in the count. */
  limit: number;
}

/** Latest verification of a gap: verdict, counts and the newest candidate papers. */
export function CheckResult({ check, limit }: Props) {
  return (
    <div className="check">
      <div className="check-head">
        <span className="verdict-pill" data-verdict={check.verdict}>{VERDICT_LABEL[check.verdict]}</span>
        <span className="muted">
          {formatInt(check.matching)} dari {formatInt(check.found)} paper baru memuat semua kata kunci · dicek {localTime(check.checkedAt)}
        </span>
      </div>
      <div className="muted check-query">Kueri: <span className="num">{check.query}</span></div>
      {check.papers.length ? (
        <ul className="check-papers">
          {check.papers.slice(0, limit).map((p, i) => (
            <li key={i}>
              <span className="check-mark" data-match={p.matches}>{p.matches ? "cocok" : "terkait"}</span>
              <span className="check-paper">
                <span>{p.title}</span>
                <span className="muted">
                  {p.year ?? "-"}
                  {p.source ? ` · ${p.source}` : ""}
                  {p.doi ? " · DOI " : ""}
                  {p.doi ? <span className="num select">{p.doi}</span> : null}
                </span>
              </span>
            </li>
          ))}
        </ul>
      ) : (
        <p className="muted">Tidak ada paper baru di luar koleksi untuk kueri ini.</p>
      )}
    </div>
  );
}
