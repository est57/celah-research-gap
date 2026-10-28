export type ScreenId =
  | "workflow"
  | "collection"
  | "overview"
  | "sources"
  | "authors"
  | "countries"
  | "conceptual"
  | "intellectual"
  | "social"
  | "opportunities"
  | "gaps"
  | "verify"
  | "report"
  | "settings";

export interface NavItem {
  id: ScreenId;
  label: string;
  showsTotal?: boolean;
}

export interface NavSection {
  title: string;
  items: NavItem[];
}

export const NAV: NavSection[] = [
  { title: "Data", items: [{ id: "workflow", label: "Alur kerja" }, { id: "collection", label: "Koleksi & impor", showsTotal: true }] },
  {
    title: "Analisis deskriptif",
    items: [
      { id: "overview", label: "Ringkasan" },
      { id: "sources", label: "Sumber" },
      { id: "authors", label: "Penulis" },
      { id: "countries", label: "Negara & kolaborasi" },
    ],
  },
  {
    title: "Struktur pengetahuan",
    items: [
      { id: "conceptual", label: "Konseptual" },
      { id: "intellectual", label: "Intelektual" },
      { id: "social", label: "Sosial" },
    ],
  },
  {
    title: "Pencarian celah",
    items: [
      { id: "opportunities", label: "Tema peluang" },
      { id: "gaps", label: "Gap Explorer" },
      { id: "verify", label: "Verifikasi" },
    ],
  },
  { title: "Keluaran", items: [{ id: "report", label: "Laporan & ekspor" }] },
  { title: "Aplikasi", items: [{ id: "settings", label: "Pengaturan" }] },
];

