import type { CitedEntities } from "../lib/apiEntities";

const rows = (list: [string, number][]) => list.map(([label, documents]) => ({ label, documents }));

export const SAMPLE_CITED_ENTITIES: CitedEntities = {
  references: 14820,
  sources: rows([["Radiology", 121], ["IEEE Access", 96], ["PLoS One", 88], ["Sci Rep", 71], ["Lancet Infect Dis", 64], ["Int J Tuberc Lung Dis", 58],
    ["Quant Imaging Med Surg", 41], ["Comput Biol Med", 37], ["Diagnostics", 33], ["Nature", 29]]),
  authors: rows([["Wang", 136], ["Rajpurkar", 121], ["Rahman", 98], ["World", 90], ["Hwang", 76], ["Jaeger", 71], ["Li", 66], ["Qin", 61],
    ["Lakhani", 58], ["Liu", 55]]),
};
