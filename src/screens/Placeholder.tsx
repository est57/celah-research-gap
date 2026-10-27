import { screenLabel, type ScreenId } from "../nav";

export function Placeholder({ id }: { id: ScreenId }) {
  return (
    <div className="placeholder">
      <div>
        <strong style={{ fontSize: 15 }}>{screenLabel(id)}</strong>
        <span className="muted">
          Layar ini sudah ada di desain, tetapi belum dibangun di versi ini. Mulai dari Koleksi &amp; impor
          untuk mengumpulkan dokumen.
        </span>
      </div>
    </div>
  );
}
