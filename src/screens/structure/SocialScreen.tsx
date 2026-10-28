import { Tabs, type TabDef } from "../../components/shell/Tabs";
import { NetworkView } from "./NetworkView";
import { t, tk } from "../../i18n";

const TABS: TabDef<"authors">[] = [{ id: "authors", label: tk("Kolaborasi penulis") }];

/** Social structure: who writes with whom. Country collaboration lives under Negara & kolaborasi. */
export function SocialScreen({ version }: { version: number }) {
  return (
    <>
      <Tabs label={t("Struktur sosial")} tabs={TABS.map((x) => ({ ...x, label: t(x.label) }))} active="authors" onChange={() => undefined} />
      <NetworkView version={version} unit="authors" />
    </>
  );
}
