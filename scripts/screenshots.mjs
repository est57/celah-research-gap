// Takes the README screenshots from the browser preview (sample data).
// Start the preview first with `npm run dev`, then run `node scripts/screenshots.mjs`.
// Images are written to docs/screenshots/.
import { mkdirSync } from "node:fs";
import { chromium } from "@playwright/test";

const BASE = "http://127.0.0.1:1420";
const OUT = "docs/screenshots";

// [file name, menu label, tab label or null, theme]
const SHOTS = [
  ["alur-kerja", "Alur kerja", null, "light"],
  ["ringkasan", "Ringkasan", null, "light"],
  ["peta-tematik", "Konseptual", null, "light"],
  ["negara", "Negara & kolaborasi", "Kolaborasi negara", "light"],
  ["tema-peluang", "Tema peluang", null, "light"],
  ["gap-explorer", "Gap Explorer", null, "dark"],
  ["laporan", "Laporan & ekspor", null, "light"],
];

// The preview marks its sample data and browser mode, and its collection count is 0;
// the screenshots show the app itself, so those parts are hidden and the status bar row
// (26 px at the bottom) is cut off.
const HIDE_PREVIEW = ".notice, .nav-item .count, span.muted.opt-hint { display: none !important; }";
const CLIP = { x: 0, y: 0, width: 1440, height: 900 - 26 };

async function click(page, selector, text) {
  await page.evaluate(([sel, t]) => {
    const el = [...document.querySelectorAll(sel)].find((e) => e.textContent?.trim().startsWith(t));
    if (!el) throw new Error(`not found: ${sel} "${t}"`);
    el.click();
  }, [selector, text]);
  await page.waitForTimeout(900);
}

mkdirSync(OUT, { recursive: true });
const browser = await chromium.launch();
for (const [name, screen, tab, theme] of SHOTS) {
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: 1 });
  await page.addInitScript((th) => localStorage.setItem("celah.display", JSON.stringify({ theme: th })), theme);
  await page.goto(BASE);
  await page.waitForTimeout(900);
  await page.addStyleTag({ content: HIDE_PREVIEW });
  await click(page, ".nav-item", screen);
  if (tab) await click(page, ".tab", tab);
  await page.screenshot({ path: `${OUT}/${name}.png`, clip: CLIP });
  await page.close();
  console.log(`${OUT}/${name}.png`);
}
await browser.close();
