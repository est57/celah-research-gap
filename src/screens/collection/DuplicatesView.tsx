import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useDuplicates } from "../../hooks/useDuplicates";
import { formatInt } from "../../lib/format";
import { DupPairCard } from "./DupPairCard";
import { t } from "../../i18n";

interface Props {
  version: number;
  onMerged: () => void;
}

export function DuplicatesView({ version, onMerged }: Props) {
  const { data, sample, error, loading, pairs, busy, actionError, merge, dismiss, pairKey } = useDuplicates(version, onMerged);
  if (error) return <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  if (!data) return <ScreenMessage>{loading ? t("Mencari dokumen yang mirip...") : ""}</ScreenMessage>;
  if (data.documents === 0) return <EmptyCollection />;

  return (
    <div className="dup-page">
      {sample ? <div className="notice">{t("Pratinjau browser: pasangan di halaman ini adalah data contoh.")}</div> : null}
      <div className="dup-summary">
        <strong>{pairs.length ? t("{n} pasangan kandidat duplikat", { n: formatInt(pairs.length) }) : t("Tidak ada kandidat duplikat")}</strong>
        <span className="muted">
          {t("dari {n} dokumen. Duplikat persis (ID, DOI, judul dan tahun sama) sudah dilewati saat impor; di sini tersisa versi preprint dan judul yang hampir sama.",
            { n: formatInt(data.documents) })}
        </span>
        {data.truncated ? <span className="muted">{t("Hanya 300 pasangan teratas yang ditampilkan.")}</span> : null}
      </div>
      {actionError ? <div className="notice" role="alert">{actionError}</div> : null}
      {pairs.length ? (
        <ul className="dup-list">
          {pairs.map((p) => (
            <DupPairCard key={pairKey(p)} pair={p} busy={busy === pairKey(p)}
              onMerge={(keep) => void merge(p, keep)} onDismiss={() => void dismiss(p)} />
          ))}
        </ul>
      ) : null}
    </div>
  );
}
