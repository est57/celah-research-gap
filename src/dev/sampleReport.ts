// Sample report for the browser preview only (npm run dev without Tauri).
// Imported lazily behind import.meta.env.DEV, so it is left out of production builds. Made-up data.
import type { Report } from "../lib/reportTypes";

const references = [
  "Chen, L., Okafor, C., & Rahman, M. (2024). Cough audio classification for pulmonary TB triage. Contoh Jurnal. https://doi.org/10.0000/contoh.2",
  "Rahman, M., Chen, L., Putri, A., Lee, J., & Silva, R. (2025). Audio spectrogram transformer for TB cough screening. Contoh Jurnal. https://doi.org/10.0000/contoh.1",
  "Silva, R., & Putri, A. (2023). Drug-resistant tuberculosis prediction from genomic data. Contoh Prosiding.",
];

export const SAMPLE_REPORT: Report = {
  title: "Celah penelitian",
  intro: "Analisis bibliometrik terhadap 553 dokumen, dengan kata kunci penulis sebagai dasar analisis konseptual, menunjukkan 2 celah penelitian yang layak dikaji lebih lanjut.",
  paragraphs: [
    {
      key: "matrix:transformer:cough-sound",
      heading: "transformer dan cough sound jarang diteliti bersama",
      text: "Meskipun transformer dan cough sound sama-sama banyak diteliti, studi yang menggabungkan keduanya masih jarang (Chen et al., 2024). Studi yang paling dekat (Rahman et al., 2025) belum menjawab persoalan ini secara langsung. Penelusuran literatur terbaru menemukan 1 studi yang mulai menyentuh aspek ini dan perlu dikaji lebih lanjut. Oleh karena itu, penelitian ini dapat diarahkan untuk menjawab bagaimana temuan dan pendekatan pada transformer dapat diterapkan untuk cough sound.",
    },
    {
      key: "niche:drug-resistant-tuberculosis",
      heading: "Tema drug-resistant tuberculosis berkembang sendiri, terpisah dari tema utama",
      text: "Kajian drug-resistant tuberculosis sudah cukup matang di dalam kelompoknya sendiri, tetapi jarang bersinggungan dengan tuberculosis detection (Silva & Putri, 2023). Oleh karena itu, penelitian ini dapat diarahkan untuk menjawab bagaimana drug-resistant tuberculosis dapat dikaitkan dengan tuberculosis detection.",
    },
  ],
  referencesTitle: "Daftar pustaka",
  references,
  bibtex: "@article{rahman2025audio,\n  author = {Rahman, M. and Chen, L. and Putri, A. and Lee, J. and Silva, R.},\n  title = {{Audio spectrogram transformer for TB cough screening}},\n  journal = {Contoh Jurnal},\n  year = {2025},\n  doi = {10.0000/contoh.1},\n}\n",
  ris: "TY  - JOUR\nAU  - Rahman, M.\nAU  - Chen, L.\nTI  - Audio spectrogram transformer for TB cough screening\nT2  - Contoh Jurnal\nPY  - 2025\nDO  - 10.0000/contoh.1\nER  - \n",
  markdown: "# Celah penelitian\n",
};
