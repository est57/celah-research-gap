import { tk } from "./i18n";

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
  { title: tk("Data"), items: [{ id: "workflow", label: tk("Alur kerja") }, { id: "collection", label: tk("Koleksi & impor"), showsTotal: true }] },
  {
    title: tk("Analisis deskriptif"),
    items: [
      { id: "overview", label: tk("Ringkasan") },
      { id: "sources", label: tk("Sumber") },
      { id: "authors", label: tk("Penulis") },
      { id: "countries", label: tk("Negara & kolaborasi") },
    ],
  },
  {
    title: tk("Struktur pengetahuan"),
    items: [
      { id: "conceptual", label: tk("Konseptual") },
      { id: "intellectual", label: tk("Intelektual") },
      { id: "social", label: tk("Sosial") },
    ],
  },
  {
    title: tk("Pencarian celah"),
    items: [
      { id: "opportunities", label: tk("Tema peluang") },
      { id: "gaps", label: tk("Gap Explorer") },
      { id: "verify", label: tk("Verifikasi") },
    ],
  },
  { title: tk("Keluaran"), items: [{ id: "report", label: tk("Laporan & ekspor") }] },
  { title: tk("Aplikasi"), items: [{ id: "settings", label: tk("Pengaturan") }] },
];

