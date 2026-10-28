import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useFilter } from "../../hooks/useFilter";
import { isDesktop } from "../../lib/api";
import { isActive, NO_FILTER } from "../../lib/filterTypes";
import { docTypeLabel, formatInt } from "../../lib/format";
import { FilterChecklist } from "./FilterChecklist";
import { t } from "../../i18n";

interface Props {
  version: number;
  onApplied: () => void;
}

const year = (v: string) => (v.trim() === "" ? null : Math.round(Number(v)));

export function FilterView({ version, onApplied }: Props) {
  const { options, sample, error, loading, draft, setDraft, preview, saving, saveError, apply } = useFilter(version, onApplied);
  if (error) return <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  if (!options) return <ScreenMessage>{loading ? t("Memuat pilihan filter...") : ""}</ScreenMessage>;
  if (options.total === 0) return <EmptyCollection />;

  const yearsOk = [draft.yearFrom, draft.yearTo].every((y) => y === null || (Number.isInteger(y) && y >= 1900 && y <= 2100))
    && (draft.yearFrom === null || draft.yearTo === null || draft.yearFrom <= draft.yearTo);
  const changed = JSON.stringify(draft) !== JSON.stringify(options.filter);
  const count = preview ?? (changed ? null : options.matching);

  return (
    <div className="filter-page">
      {sample ? <div className="notice">{t("Pratinjau browser: pilihan di halaman ini adalah data contoh dan tidak bisa disimpan.")}</div> : null}
      <div className="filter-summary">
        <strong>{isActive(options.filter)
          ? t("Analisis memakai {n} dari {total} dokumen", { n: formatInt(options.matching), total: formatInt(options.total) })
          : t("Analisis memakai seluruh {total} dokumen", { total: formatInt(options.total) })}</strong>
        <span className="muted">{t("Filter berlaku untuk semua analisis, Gap Explorer, verifikasi, dan laporan. Daftar koleksi, kualitas metadata, dan duplikat tetap menampilkan seluruh koleksi.")}</span>
      </div>
      <div className="filter-grid">
        <div className="filter-col">
          <section className="pnl filter-group">
            <div className="ph">{t("Tahun terbit")}<span className="ph-aside">{t("{from} sampai {to}", { from: options.yearMin ?? "-", to: options.yearMax ?? "-" })}</span></div>
            <div className="filter-years">
              <label htmlFor="f-from">{t("Dari")}</label>
              <input id="f-from" className="input num" type="number" min={1900} max={2100} placeholder={String(options.yearMin ?? "")}
                value={draft.yearFrom ?? ""} onChange={(e) => setDraft({ ...draft, yearFrom: year(e.target.value) })} />
              <label htmlFor="f-to">{t("Sampai")}</label>
              <input id="f-to" className="input num" type="number" min={1900} max={2100} placeholder={String(options.yearMax ?? "")}
                value={draft.yearTo ?? ""} onChange={(e) => setDraft({ ...draft, yearTo: year(e.target.value) })} />
            </div>
            {!yearsOk ? <p className="filter-error" role="alert">{t("Tahun harus 1900 sampai 2100, dan \"dari\" tidak boleh setelah \"sampai\".")}</p> : null}
          </section>
          <FilterChecklist id="f-type" title={t("Jenis dokumen")} choices={options.docTypes} selected={draft.docTypes}
            label={(v) => (v ? docTypeLabel(v) : t("Tidak diketahui"))} onChange={(docTypes) => setDraft({ ...draft, docTypes })} />
          <FilterChecklist id="f-origin" title={t("Sumber impor")} choices={options.origins} selected={draft.origins}
            label={(v) => v} onChange={(origins) => setDraft({ ...draft, origins })} />
          {options.quartiles.length ? (
            <FilterChecklist id="f-quartile" title={t("Kuartil jurnal (SJR)")} choices={options.quartiles} selected={draft.quartiles}
              label={(v) => (v === "none" ? t("Tanpa kuartil") : v)} onChange={(quartiles) => setDraft({ ...draft, quartiles })} />
          ) : (
            <p className="muted filter-note">{t("Impor data SJR di Sumber, tab Kuartil (SJR), untuk menyaring menurut kuartil jurnal.")}</p>
          )}
        </div>
        <FilterChecklist id="f-source" title={t("Sumber terbitan (jurnal, prosiding)")} choices={options.sources} selected={draft.sources}
          label={(v) => v || t("Tanpa nama sumber")} searchable onChange={(sources) => setDraft({ ...draft, sources })} />
      </div>
      <div className="filter-bar">
        <span className="filter-count" role="status">
          {count === null ? t("Menghitung...") : t("{n} dari {total} dokumen lolos filter", { n: formatInt(count), total: formatInt(options.total) })}
          {count === 0 ? <span className="filter-warn">{t("; analisis akan kosong")}</span> : null}
        </span>
        {saveError ? <span className="filter-error" role="alert">{saveError}</span> : null}
        <button type="button" className="btn" disabled={saving || !isDesktop()} onClick={() => void apply(NO_FILTER)}>{t("Hapus filter")}</button>
        <button type="button" className="btn btn-primary" disabled={saving || !yearsOk || !changed || !isDesktop()} onClick={() => void apply(draft)}>
          {saving ? t("Menerapkan...") : t("Terapkan filter")}
        </button>
      </div>
    </div>
  );
}
