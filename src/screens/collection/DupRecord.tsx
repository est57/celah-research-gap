import { docTypeLabel, formatInt } from "../../lib/format";
import type { DupPaper } from "../../lib/duplicateTypes";
import { t, tk } from "../../i18n";

// Values match paper.has from the backend; translated only where shown.
const FIELDS = [tk("Abstrak"), tk("Kata kunci"), tk("Referensi"), tk("Negara")];

interface Props {
  paper: DupPaper;
  name: string;
  kept: boolean;
  disabled: boolean;
  onKeep: () => void;
}

export function DupRecord({ paper, name, kept, disabled, onKeep }: Props) {
  const meta = [paper.year ?? t("t.t."), paper.origin, paper.docType ? docTypeLabel(paper.docType) : null].filter(Boolean).join(" · ");
  return (
    <div className="dup-rec" data-kept={kept}>
      <label className="dup-keep">
        <input type="radio" name={name} checked={kept} disabled={disabled} onChange={onKeep} />
        <span>{kept ? t("Dipertahankan") : t("Dihapus saat digabung")}</span>
      </label>
      <p className="dup-title">{paper.title}</p>
      <p className="dup-meta">{paper.authors || t("Penulis tidak tercatat")}</p>
      <p className="dup-meta">{meta}</p>
      {paper.source ? <p className="dup-meta dup-source">{paper.source}</p> : null}
      <p className="dup-meta num">{paper.doi ?? t("tanpa DOI")}</p>
      <div className="dup-fields" aria-label={t("Kelengkapan field")}>
        {FIELDS.map((f) => (
          <span key={f} className="dup-field" data-on={paper.has.includes(f)}>{t(f)}</span>
        ))}
        <span className="dup-cites num">{t("{n} sitasi", { n: formatInt(paper.citedBy) })}</span>
      </div>
    </div>
  );
}
