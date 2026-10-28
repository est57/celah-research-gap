import { CompleteTextRefs } from "./CompleteTextRefs";
import { ScreenMessage } from "./ScreenMessage";
import { fill } from "./fill";
import { t } from "../i18n";

/**
 * Shown when the collection has documents but none carry a reference list, which is
 * what a Scopus export without the References field looks like.
 */
export function NoReferences({ analysis, onDone }: { analysis: string; onDone?: () => void }) {
  return (
    <ScreenMessage>
      <strong>{t("Dokumen di koleksi belum punya daftar rujukan")}</strong>
      <span className="muted refs-help">
        {t("{analysis} dihitung dari daftar rujukan tiap dokumen. Ekspor Scopus hanya menyertakannya bila kolom References dicentang.", { analysis })}
      </span>
      {onDone ? <CompleteTextRefs onDone={onDone} /> : null}
      <span className="muted refs-help">{t("Atau ekspor ulang dari Scopus:")}</span>
      <ol className="refs-steps">
        <li>{t("Di Scopus, pilih dokumen lalu klik Export dan pilih CSV atau BibTeX.")}</li>
        <li>{fill(t("Centang {field} (dan Abstract, Author keywords, Affiliations)."), { field: <strong>{"References"}</strong> })}</li>
        <li>{t("Impor file itu lagi di Koleksi & impor. Dokumen yang sama dilengkapi, tidak digandakan.")}</li>
      </ol>
      <span className="muted refs-help">
        {t("Web of Science (Full record and cited references) dan pencarian Crossref juga menyertakan rujukan. Rujukan dari OpenAlex (Historiograf, Lengkapi rujukan) hanya berupa ID tanpa tahun, jadi cukup untuk Historiograf dan Kopling, tetapi tidak untuk Ko-sitasi, Rujukan teratas, dan Akar sejarah.")}
      </span>
    </ScreenMessage>
  );
}
