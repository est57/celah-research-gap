import { useEffect, useState } from "react";
import { countryName } from "../../lib/countryNames";
import type { CountryCount, CountryPair } from "../../lib/countriesTypes";
import { formatInt } from "../../lib/format";
import { t } from "../../i18n";

interface World {
  width: number;
  height: number;
  countries: { code: string; d: string }[];
  centers: Record<string, number[]>;
}

interface Props {
  production: CountryCount[];
  pairs: CountryPair[];
}

/** Scientific production per country (fill) and the strongest collaborations (arcs). */
export function WorldMap({ production, pairs }: Props) {
  const [world, setWorld] = useState<World | null>(null);
  useEffect(() => {
    // Outlines are bundled data, loaded only when the map is shown.
    let alive = true;
    void import("../../data/world.json").then((m) => { if (alive) setWorld(m.default); });
    return () => { alive = false; };
  }, []);
  if (!world) return <div className="wmap-loading muted">{t("Memuat peta...")}</div>;

  const count = new Map(production.map((c) => [c.code, c.count]));
  const max = Math.max(1, ...production.map((c) => c.count));
  const maxPair = Math.max(1, ...pairs.map((p) => p.count));
  const share = (code: string) => Math.sqrt((count.get(code) ?? 0) / max);

  return (
    <svg className="wmap" viewBox={`0 0 ${world.width} ${world.height}`} role="img"
      aria-label={t("Peta produksi ilmiah negara dan kolaborasi terkuat")}>
      {world.countries.map((c, i) => {
        const n = count.get(c.code);
        return (
          <path key={`${c.code}-${i}`} d={c.d} className="wmap-land" data-has={n !== undefined}
            style={n !== undefined ? { fillOpacity: 0.25 + 0.75 * share(c.code) } : undefined}>
            <title>{!c.code ? "" : n !== undefined ? t("{country}: {n} dokumen", { country: countryName(c.code), n: formatInt(n) }) : countryName(c.code)}</title>
          </path>
        );
      })}
      {pairs.map((p) => {
        const a = world.centers[p.a], b = world.centers[p.b];
        if (!a || !b) return null;
        // Arcs bow upward in proportion to their length.
        const mx = (a[0] + b[0]) / 2, my = (a[1] + b[1]) / 2 - Math.hypot(b[0] - a[0], b[1] - a[1]) * 0.25;
        return (
          <path key={`${p.a}-${p.b}`} d={`M${a[0]} ${a[1]} Q${mx} ${my} ${b[0]} ${b[1]}`} className="wmap-arc"
            style={{ strokeWidth: 0.8 + 3 * (p.count / maxPair) }}>
            <title>{t("{a} dan {b}: {n} dokumen bersama", { a: countryName(p.a), b: countryName(p.b), n: formatInt(p.count) })}</title>
          </path>
        );
      })}
      {production.map((c) => {
        const at = world.centers[c.code];
        return at ? <circle key={c.code} cx={at[0]} cy={at[1]} r={2 + 6 * share(c.code)} className="wmap-dot" /> : null;
      })}
    </svg>
  );
}
