import type { SourceId } from "../../lib/types";
import { t, tk } from "../../i18n";

export const SOURCES: { id: SourceId; label: string; note: string }[] = [
  { id: "openalex", label: "OpenAlex", note: tk("Cakupan terluas, ada afiliasi dan kata kunci") },
  { id: "crossref", label: "Crossref", note: tk("Metadata DOI resmi, sering memuat referensi") },
  { id: "semantic_scholar", label: "Semantic Scholar", note: tk("Perlu kunci API agar tidak cepat dibatasi") },
  { id: "pubmed", label: "PubMed", note: tk("Biomedis dan kesehatan, ada istilah MeSH dan referensi") },
  { id: "europe_pmc", label: "Europe PMC", note: tk("Biomedis, termasuk pra-cetak dan jumlah sitasi") },
  { id: "arxiv", label: "arXiv", note: tk("Pra-cetak AI, ilmu komputer, fisika, matematika. Lambat: jeda 3 detik per halaman") },
];

interface Props {
  value: SourceId[];
  onChange: (next: SourceId[]) => void;
}

export function SourcePicker({ value, onChange }: Props) {
  const toggle = (id: SourceId) =>
    onChange(value.includes(id) ? value.filter((v) => v !== id) : [...value, id]);

  return (
    <fieldset className="source-picker">
      <legend>{t("Sumber data")}</legend>
      {SOURCES.map((s) => (
        <label key={s.id} className="source-option" title={t(s.note)}>
          <input type="checkbox" checked={value.includes(s.id)} onChange={() => toggle(s.id)} />
          {s.label}
        </label>
      ))}
    </fieldset>
  );
}
