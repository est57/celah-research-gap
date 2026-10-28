import { formatDecimal, formatInt } from "../../lib/format";
import { DEFAULT_DISPLAY, SCALES, type DisplaySettings } from "../../lib/display";
import { t } from "../../i18n";

interface Props {
  settings: DisplaySettings;
  onChange: (patch: Partial<DisplaySettings>) => void;
}

function Row<T extends string | number>({ label, hint, value, options, onPick }: { label: string; hint: string; value: T; options: [T, string][]; onPick: (v: T) => void }) {
  return (
    <div className="disp-row">
      <div className="disp-text">
        <strong>{label}</strong>
        <span className="muted">{hint}</span>
      </div>
      <div className="opt-seg disp-seg" role="group" aria-label={label}>
        {options.map(([v, text]) => <button key={String(v)} type="button" aria-pressed={value === v} onClick={() => onPick(v)}>{text}</button>)}
      </div>
    </div>
  );
}

export function DisplayPanel({ settings: s, onChange }: Props) {
  return (
    <section className="pnl key-panel">
      <div className="ph">{t("Tampilan")}</div>
      <Row label={t("Bahasa antarmuka")} hint={t("Menu, tombol, pesan, dan penjelasan celah.")} value={s.language}
        options={[["id", "Indonesia"], ["en", "English"]]} onPick={(language) => onChange({ language })} />
      <Row label={t("Tema")} hint={t("Ikuti sistem mengikuti mode terang atau gelap di sistem operasi.")} value={s.theme}
        options={[["system", t("Ikuti sistem")], ["light", t("Terang")], ["dark", t("Gelap")]]} onPick={(theme) => onChange({ theme })} />
      <Row label={t("Ukuran tampilan")} hint={t("Memperbesar atau memperkecil seluruh aplikasi.")} value={s.scale}
        options={SCALES.map((v) => [v, `${Math.round(v * 100)}%`])} onPick={(scale) => onChange({ scale })} />
      <Row label={t("Format angka")} hint={t("Contoh: {n} dokumen, {rate} sitasi per tahun.", { n: formatInt(12345), rate: formatDecimal(3.5, 1) })} value={s.numbers}
        options={[["id", "1.234,5"], ["en", "1,234.5"]]} onPick={(numbers) => onChange({ numbers })} />
      <Row label={t("Animasi")} hint={t("Kurangi gerakan seperti geser menu dan transisi.")} value={s.reduceMotion ? "off" : "on"}
        options={[["on", t("Normal")], ["off", t("Kurangi")]]} onPick={(v) => onChange({ reduceMotion: v === "off" })} />
      <div className="ph disp-sub">{t("Bawaan laporan")}</div>
      <Row label={t("Gaya sitasi")} hint={t("Dipakai saat membuka Laporan & ekspor.")} value={s.reportStyle}
        options={[["apa", "APA 7"], ["ieee", "IEEE"]]} onPick={(reportStyle) => onChange({ reportStyle })} />
      <Row label={t("Bahasa draf")} hint={t("Bahasa paragraf research gap yang disusun.")} value={s.reportLanguage}
        options={[["id", "Indonesia"], ["en", "English"]]} onPick={(reportLanguage) => onChange({ reportLanguage })} />
      <div className="disp-foot">
        <span className="muted">{t("Pengaturan tampilan disimpan di perangkat ini.")}</span>
        <button type="button" className="btn" onClick={() => onChange(DEFAULT_DISPLAY)}>{t("Kembalikan bawaan")}</button>
      </div>
    </section>
  );
}
