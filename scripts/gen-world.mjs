// Builds src/data/world.json: country outlines as SVG paths in the Equal Earth projection,
// plus a center point for every country, from the Natural Earth data in world-atlas.
// Run once with: node scripts/gen-world.mjs (needs the dev dependencies installed).
import { readFileSync, writeFileSync } from "node:fs";
import { feature } from "topojson-client";

const read = (f) => JSON.parse(readFileSync(new URL(`../node_modules/world-atlas/${f}`, import.meta.url)));
const W = 1000;
const [A1, A2, A3, A4] = [1.340264, -0.081106, 0.000893, 0.003796];
const M = Math.sqrt(3) / 2;

function project([lon, lat]) {
  const l = (lon * Math.PI) / 180, t = Math.asin(M * Math.sin((lat * Math.PI) / 180));
  const t2 = t * t, t6 = t2 * t2 * t2;
  const x = (l * Math.cos(t)) / (M * (A1 + 3 * A2 * t2 + t6 * (7 * A3 + 9 * A4 * t2)));
  const y = t * (A1 + A2 * t2 + t6 * (A3 + A4 * t2));
  return [x, y];
}
// The projection spans about +-2.7064 horizontally and +-1.3173 vertically.
const XMAX = 2.7064, YMAX = 1.3173;
const H = Math.round((W * YMAX) / XMAX);
const toSvg = ([x, y]) => [((x + XMAX) / (2 * XMAX)) * W, ((YMAX - y) / (2 * YMAX)) * H];
/** SVG path of one ring; a jump across the antimeridian starts a new subpath instead of a line. */
function ringPath(ring) {
  const pts = ring.map((c) => toSvg(project(c)));
  let d = "";
  pts.forEach(([x, y], i) => {
    const jump = i > 0 && Math.abs(x - pts[i - 1][0]) > W / 2;
    d += `${i === 0 || jump ? "M" : "L"}${x.toFixed(1)} ${y.toFixed(1)}`;
  });
  return `${d}Z`;
}

// Alpha-2 code for each numeric id, from English region names plus aliases for the atlas spellings.
const display = new Intl.DisplayNames(["en"], { type: "region" });
const byName = new Map();
for (let a = 65; a <= 90; a++) for (let b = 65; b <= 90; b++) {
  const code = String.fromCharCode(a, b);
  try { const n = display.of(code); if (n && n !== code) byName.set(n.toLowerCase(), code); } catch { /* not a region */ }
}
const ALIAS = {
  "united states of america": "US", "dem. rep. congo": "CD", "congo": "CG", "central african rep.": "CF", "bosnia and herz.": "BA",
  "s. sudan": "SS", "dominican rep.": "DO", "eq. guinea": "GQ", "solomon is.": "SB", "côte d'ivoire": "CI", "w. sahara": "EH",
  "falkland is.": "FK", "fr. s. antarctic lands": "TF", "kosovo": "XK", "macedonia": "MK", "north macedonia": "MK", "myanmar": "MM",
  "palestine": "PS", "turkey": "TR", "eswatini": "SZ", "czechia": "CZ", "timor-leste": "TL", "bahamas": "BS", "gambia": "GM",
  "s. geo. and the is.": "GS", "faeroe is.": "FO", "cayman is.": "KY", "n. mariana is.": "MP", "br. indian ocean ter.": "IO",
  "heard i. and mcdonald is.": "HM", "fr. polynesia": "PF", "wallis and futuna is.": "WF", "st. vin. and gren.": "VC",
  "st. kitts and nevis": "KN", "antigua and barb.": "AG", "u.s. virgin is.": "VI", "british virgin is.": "VG", "turks and caicos is.": "TC",
  "st. pierre and miquelon": "PM", "marshall is.": "MH", "cook is.": "CK", "são tomé and principe": "ST", "cabo verde": "CV",
  "micronesia": "FM", "pitcairn is.": "PN", "ashmore and cartier is.": "AU", "siachen glacier": "IN", "indian ocean ter.": "AU",
  "trinidad and tobago": "TT", "saint lucia": "LC", "saint helena": "SH", "åland": "AX",
  "st-martin": "MF", "st-barthélemy": "BL", "sint maarten": "SX", "curaçao": "CW", "aland": "AX", "vatican": "VA", "hong kong": "HK", "macao": "MO",
};
const codeOf = (name) => ALIAS[name.toLowerCase()] ?? byName.get(name.toLowerCase()) ?? null;

function ringArea(r) { let a = 0; for (let i = 0; i < r.length - 1; i++) a += r[i][0] * r[i + 1][1] - r[i + 1][0] * r[i][1]; return a / 2; }
/** Center of the largest polygon, in SVG units. */
function center(geom) {
  const polys = geom.type === "Polygon" ? [geom.coordinates] : geom.coordinates;
  const ring = polys.map((p) => p[0].map((c) => toSvg(project(c)))).sort((a, b) => Math.abs(ringArea(b)) - Math.abs(ringArea(a)))[0];
  let cx = 0, cy = 0, a = 0;
  for (let i = 0; i < ring.length - 1; i++) {
    const f = ring[i][0] * ring[i + 1][1] - ring[i + 1][0] * ring[i][1];
    cx += (ring[i][0] + ring[i + 1][0]) * f; cy += (ring[i][1] + ring[i + 1][1]) * f; a += f;
  }
  return a ? [+(cx / (3 * a)).toFixed(1), +(cy / (3 * a)).toFixed(1)] : ring[0].map((v) => +v.toFixed(1));
}

const outlines = feature(read("countries-110m.json"), read("countries-110m.json").objects.countries).features;
const detailed = feature(read("countries-50m.json"), read("countries-50m.json").objects.countries).features;
const countries = [], missing = [];
for (const f of outlines) {
  if (!f.geometry) continue;
  const polys = f.geometry.type === "Polygon" ? [f.geometry.coordinates] : f.geometry.coordinates;
  const d = polys.map((p) => p.map(ringPath).join("")).join("");
  const code = codeOf(f.properties.name);
  if (!code && f.properties.name !== "Antarctica") missing.push(f.properties.name);
  if (f.properties.name !== "Antarctica") countries.push({ code: code ?? "", d });
}
const centers = {};
for (const f of detailed) {
  const code = codeOf(f.properties.name);
  if (code && f.geometry && !centers[code]) centers[code] = center(f.geometry);
  else if (!code) missing.push(`(50m) ${f.properties.name}`);
}
writeFileSync(new URL("../src/data/world.json", import.meta.url), JSON.stringify({ width: W, height: H, countries, centers }));
console.log(`countries ${countries.length}, centers ${Object.keys(centers).length}, unmatched: ${missing.join("; ") || "none"}`);
