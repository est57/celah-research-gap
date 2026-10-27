import { Tabs, type TabDef } from "../../components/shell/Tabs";
import { NetworkView } from "./NetworkView";

const TABS: TabDef<"authors">[] = [{ id: "authors", label: "Kolaborasi penulis" }];

/** Social structure: who writes with whom. Country collaboration lives under Negara & kolaborasi. */
export function SocialScreen({ version }: { version: number }) {
  return (
    <>
      <Tabs label="Struktur sosial" tabs={TABS} active="authors" onChange={() => undefined} />
      <NetworkView version={version} unit="authors" />
    </>
  );
}
