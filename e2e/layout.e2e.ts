import { expect, test, type Page } from "@playwright/test";
import { findLayoutProblems } from "./layoutChecks";

// The widths the design must hold (CLAUDE.md), from wide desktop to the minimum window,
// in both interface languages: English labels are often longer than Indonesian ones.
const WIDTHS = [1440, 1100, 800];
const LANGUAGES = ["id", "en"] as const;

const settle = (page: Page) => page.waitForTimeout(600);

/** Clicks through the DOM so the closed navigation drawer at narrow widths does not matter. */
async function clickByText(page: Page, selector: string, text: string) {
  await page.evaluate(
    ([sel, t]) => {
      const el = [...document.querySelectorAll<HTMLElement>(sel)].find((e) => e.textContent?.trim().startsWith(t));
      if (!el) throw new Error(`tidak ditemukan: ${sel} "${t}"`);
      el.click();
    },
    [selector, text],
  );
  await settle(page);
}

for (const language of LANGUAGES) for (const width of WIDTHS) {
  test(`setiap halaman dan tab rapi di lebar ${width}px (${language})`, async ({ page }) => {
    await page.setViewportSize({ width, height: 900 });
    await page.addInitScript((lang) => localStorage.setItem("celah.display", JSON.stringify({ language: lang })), language);
    await page.goto("/");
    await settle(page);
    const screens = await page.$$eval(".nav-item", (items) => items.map((i) => (i.firstChild?.textContent ?? i.textContent ?? "").trim()));
    expect(screens.length).toBeGreaterThan(10);
    const problems: string[] = [];
    for (const screen of screens) {
      await clickByText(page, ".nav-item", screen);
      const tabs = await page.$$eval(".tab", (els) => els.map((e) => (e.textContent ?? "").trim()));
      for (const tab of tabs.length ? tabs : [""]) {
        if (tab) await clickByText(page, ".tab", tab);
        const found = await page.evaluate(findLayoutProblems);
        problems.push(...found.map((p) => `${screen}${tab ? ` > ${tab}` : ""}: ${p}`));
      }
    }
    expect(problems, problems.join("\n")).toEqual([]);
  });
}
