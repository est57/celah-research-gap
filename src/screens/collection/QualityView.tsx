import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { useQuality } from "../../hooks/useQuality";
import { FieldMatrix } from "./FieldMatrix";
import { ReadinessPanel } from "./ReadinessPanel";
import { t } from "../../i18n";

export function QualityView({ version }: { version: number }) {
  const { data, sample, error, loading } = useQuality(version);
  if (error) return <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  if (!data) return <ScreenMessage>{loading ? t("Memeriksa kelengkapan metadata...") : ""}</ScreenMessage>;
  if (data.total === 0) return <EmptyCollection />;
  return (
    <div className="quality-page">
      {sample ? <div className="notice">{t("Pratinjau browser: angka di halaman ini adalah data contoh.")}</div> : null}
      <div className="quality-grid">
        <ReadinessPanel quality={data} />
        <FieldMatrix quality={data} />
      </div>
    </div>
  );
}
