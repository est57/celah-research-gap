// Sample networks for the browser preview only (npm run dev without Tauri).
// Imported lazily behind import.meta.env.DEV, so they are left out of production builds. Made-up data.
import type { NetEdge, NetNode, Structure, Unit } from "../lib/structureTypes";

const GROUPS: Record<Unit, string[][]> = {
  references: [
    ["Qin 2019", "Lakhani 2017", "Who 2021", "Jaeger 2014", "Hwang 2019", "Murphy 2020", "Khan 2020"],
    ["Boehme 2010", "Steingart 2014", "Dorman 2018", "Lawn 2011", "Pai 2016"],
    ["He 2016", "Rajpurkar 2017", "Dosovitskiy 2021", "Simonyan 2015", "Selvaraju 2017", "Krizhevsky 2012"],
    ["Botha 2018", "Pahar 2021", "Zimmer 2022"],
  ],
  authors: [
    ["Rahman M", "Chen L", "Kumar S", "Lee J", "Nguyen T"],
    ["Mensah K", "Mbeki T", "Santos P", "Qadir S"],
    ["Okafor C", "Silva R", "Pratama A", "Wijaya D"],
    ["Hartono B", "Putri N", "Sari D"],
  ],
  coupling_documents: [
    ["Pham 2022", "Pham 2021", "Anis 2020", "Rahman 2020", "Hwang 2019"],
    ["Hansun 2023", "Hansun 2022", "Oloko-Oba 2022", "Kotei 2022"],
    ["Owda 2025", "Sharma 2024", "Mirugwe 2024"],
    ["Pahar 2021", "Zimmer 2022", "Botha 2018"],
  ],
  coupling_sources: [
    ["Diagnostics", "IEEE Access", "Scientific Reports", "PLOS ONE"],
    ["Journal of Medical Internet Research", "Frontiers in Medicine", "Healthcare"],
    ["Radiology", "European Radiology", "Tuberculosis"],
    ["Physiological Measurement", "Computers in Biology and Medicine"],
  ],
  coupling_authors: [
    ["Hansun S", "Argha A", "Celler BG", "Liaw ST"],
    ["Nguyen HQ", "Le LT", "Nguyen HT"],
    ["Park CM", "Goo JM", "Hwang EJ"],
    ["Pahar M", "Niesler T"],
  ],
  keywords: [
    ["deep learning", "chest x-ray", "convolutional neural network", "transfer learning", "computer-aided detection"],
    ["tuberculosis", "screening", "diagnosis", "sputum smear", "GeneXpert"],
    ["drug-resistant tuberculosis", "whole genome sequencing", "machine learning"],
    ["cough sound", "audio classification", "smartphone"],
  ],
};

export function sampleStructure(unit: Unit): Structure {
  const nodes: NetNode[] = [];
  const edges: NetEdge[] = [];
  const starts: number[] = [];
  GROUPS[unit].forEach((group, cluster) => {
    const start = nodes.length;
    starts.push(start);
    group.forEach((label, i) => nodes.push({ label, count: 40 - i * 4 - cluster * 3, cluster }));
    for (let i = start; i < nodes.length; i++) {
      for (let j = i + 1; j < nodes.length; j++) {
        if ((i + j) % 3 !== 0) edges.push({ from: i, to: j, count: 6, weight: 0.4 + ((i * j) % 5) / 10 });
      }
    }
  });
  // A few links between clusters; the last cluster stays almost isolated.
  edges.push(
    { from: 0, to: starts[1], count: 3, weight: 0.2 },
    { from: 2, to: starts[2], count: 4, weight: 0.25 },
    { from: starts[2] + 1, to: nodes.length - 1, count: 1, weight: 0.05 },
  );
  return {
    unit: { references: "Referensi tersitasi", authors: "Penulis", keywords: "Kata kunci penulis",
      coupling_documents: "Dokumen", coupling_sources: "Sumber", coupling_authors: "Penulis" }[unit],
    documents: 1836,
    network: {
      nodes,
      edges,
      clusters: GROUPS[unit].map((g, id) => ({ id, size: g.length, top: g.slice(0, 3) })),
      bridges: [{ a: 0, b: 3, links: 1, ratio: 0.08 }, { a: 1, b: 3, links: 0, ratio: 0 }],
    },
  };
}
