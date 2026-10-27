// Sample data for the browser preview only (npm run dev without Tauri).
// Imported lazily behind import.meta.env.DEV, so it is left out of production builds. Made-up numbers.
import type { Countries } from "../lib/countriesTypes";

const corresponding: [string, number, number][] = [
  ["IN", 241, 71], ["CN", 219, 49], ["US", 92, 72], ["ZA", 68, 53], ["GB", 31, 47], ["ID", 39, 19], ["BR", 28, 14], ["VN", 17, 15],
];
const codes = ["IN", "CN", "US", "ZA", "GB", "ID", "BR", "VN", "NL", "DE", "KE", "NG"];
const clusterOf: Record<string, number> = { IN: 0, US: 0, CN: 0, GB: 1, ZA: 1, KE: 1, NG: 1, ID: 2, NL: 2, VN: 2, BR: 3, DE: 3 };
const links: [string, string, number][] = [
  ["IN", "US", 38], ["ZA", "GB", 31], ["CN", "US", 29], ["IN", "GB", 22], ["ID", "NL", 12], ["KE", "GB", 11],
  ["NG", "GB", 9], ["VN", "NL", 7], ["BR", "DE", 6], ["US", "GB", 14], ["IN", "CN", 5], ["ID", "VN", 4],
];

export const SAMPLE_COUNTRIES: Countries = {
  documentsWithCountry: 1702,
  internationalShare: 0.386,
  corresponding: corresponding.map(([code, scp, mcp]) => ({ code, scp, mcp, articles: scp + mcp })),
  production: codes.map((code, i) => ({ code, count: 420 - i * 30 })),
  pairs: links.slice(0, 8).map(([a, b, count]) => ({ a, b, count })),
  network: {
    nodes: codes.map((code, i) => ({ label: code, count: 420 - i * 30, cluster: clusterOf[code] })),
    edges: links.map(([a, b, count]) => ({ from: codes.indexOf(a), to: codes.indexOf(b), count, weight: count / 38 })),
    clusters: [0, 1, 2, 3].map((id) => ({ id, size: codes.filter((c) => clusterOf[c] === id).length, top: codes.filter((c) => clusterOf[c] === id).slice(0, 3) })),
    bridges: [{ a: 2, b: 3, links: 0, ratio: 0 }],
  },
};
