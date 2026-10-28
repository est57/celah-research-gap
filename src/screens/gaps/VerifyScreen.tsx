import type { ReactNode } from "react";
import { EmptyCollection, ScreenMessage } from "../../components/ScreenMessage";
import { Tabs, type TabDef } from "../../components/shell/Tabs";
import { useGaps } from "../../hooks/useGaps";
import { t, tk } from "../../i18n";
import { VerifyView } from "./VerifyView";

const TABS: TabDef<"verify">[] = [{ id: "verify", label: tk("Verifikasi literatur terbaru") }];

/** Checks accepted gaps against recent papers outside the collection. */
export function VerifyScreen({ version }: { version: number }) {
  const { data, error, loading, items, verify, verifying } = useGaps(version);
  let body: ReactNode;
  if (error) body = <ScreenMessage><span role="alert">{error}</span></ScreenMessage>;
  else if (!data) body = <ScreenMessage>{loading ? t("Memuat kandidat celah...") : ""}</ScreenMessage>;
  else if (data.documents === 0) body = <EmptyCollection />;
  else if (items.length === 0) {
    body = <ScreenMessage>{t("Belum ada kandidat celah untuk diverifikasi. Tinjau dan terima kandidat di Gap Explorer dulu.")}</ScreenMessage>;
  } else body = <VerifyView gaps={items} verifying={verifying} onVerify={verify} />;
  return (
    <>
      <Tabs label={t("Verifikasi")} tabs={TABS.map((x) => ({ ...x, label: t(x.label) }))} active="verify" onChange={() => undefined} />
      {body}
    </>
  );
}
