import { CompleteTextRefs } from "./CompleteTextRefs";
import { ScreenMessage } from "./ScreenMessage";

/**
 * Shown when the collection has documents but none carry a reference list, which is
 * what a Scopus export without the References field looks like.
 */
export function NoReferences({ analysis, onDone }: { analysis: string; onDone?: () => void }) {
  return (
    <ScreenMessage>
      <strong>Dokumen di koleksi belum punya daftar rujukan</strong>
      <span className="muted refs-help">
        {`${analysis} dihitung dari daftar rujukan tiap dokumen. Ekspor Scopus hanya menyertakannya bila kolom References dicentang.`}
      </span>
      {onDone ? <CompleteTextRefs onDone={onDone} /> : null}
      <span className="muted refs-help">Atau ekspor ulang dari Scopus:</span>
      <ol className="refs-steps">
        <li>Di Scopus, pilih dokumen lalu klik Export dan pilih CSV atau BibTeX.</li>
        <li>Centang <strong>References</strong> (dan Abstract, Author keywords, Affiliations).</li>
        <li>Impor file itu lagi di Koleksi &amp; impor. Dokumen yang sama dilengkapi, tidak digandakan.</li>
      </ol>
      <span className="muted refs-help">
        Web of Science (Full record and cited references) dan pencarian Crossref juga menyertakan rujukan. Rujukan dari
        OpenAlex (Historiograf, Lengkapi rujukan) hanya berupa ID tanpa tahun, jadi cukup untuk Historiograf dan Kopling,
        tetapi tidak untuk Ko-sitasi, Rujukan teratas, dan Akar sejarah.
      </span>
    </ScreenMessage>
  );
}
