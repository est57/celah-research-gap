import { docTypeLabel, formatInt } from "../../lib/format";
import type { DupPaper } from "../../lib/duplicateTypes";

const FIELDS = ["Abstrak", "Kata kunci", "Referensi", "Negara"];

interface Props {
  paper: DupPaper;
  name: string;
  kept: boolean;
  disabled: boolean;
  onKeep: () => void;
}

export function DupRecord({ paper, name, kept, disabled, onKeep }: Props) {
  const meta = [paper.year ?? "t.t.", paper.origin, paper.docType ? docTypeLabel(paper.docType) : null].filter(Boolean).join(" · ");
  return (
    <div className="dup-rec" data-kept={kept}>
      <label className="dup-keep">
        <input type="radio" name={name} checked={kept} disabled={disabled} onChange={onKeep} />
        <span>{kept ? "Dipertahankan" : "Dihapus saat digabung"}</span>
      </label>
      <p className="dup-title">{paper.title}</p>
      <p className="dup-meta">{paper.authors || "Penulis tidak tercatat"}</p>
      <p className="dup-meta">{meta}</p>
      {paper.source ? <p className="dup-meta dup-source">{paper.source}</p> : null}
      <p className="dup-meta num">{paper.doi ?? "tanpa DOI"}</p>
      <div className="dup-fields" aria-label="Kelengkapan field">
        {FIELDS.map((f) => (
          <span key={f} className="dup-field" data-on={paper.has.includes(f)}>{f}</span>
        ))}
        <span className="dup-cites num">{`${formatInt(paper.citedBy)} sitasi`}</span>
      </div>
    </div>
  );
}
