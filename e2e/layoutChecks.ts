// Runs inside the page. It must stay self-contained: Playwright sends only this
// function's source to the browser, so it cannot use imports or outer variables.

/** Layout problems on the current screen, as short readable lines. */
export function findLayoutProblems(): string[] {
  const problems: string[] = [];
  const root = document.documentElement;
  if (root.scrollWidth > root.clientWidth + 1) {
    problems.push(`halaman melebar ke samping: ${root.scrollWidth}px untuk layar ${root.clientWidth}px`);
  }
  const shown = (el: Element) => {
    const r = el.getBoundingClientRect();
    const s = getComputedStyle(el);
    return r.width > 0 && r.height > 0 && s.visibility !== "hidden" && s.display !== "none" && Number(s.opacity) > 0;
  };
  const label = (el: Element) => {
    const cls = typeof el.className === "string" && el.className.trim() ? `.${el.className.trim().split(/\s+/).join(".")}` : "";
    return `${el.tagName.toLowerCase()}${cls} "${(el.textContent ?? "").trim().replace(/\s+/g, " ").slice(0, 40)}"`;
  };
  const overlap = (a: DOMRect, b: DOMRect) =>
    Math.max(0, Math.min(a.right, b.right) - Math.max(a.left, b.left)) * Math.max(0, Math.min(a.bottom, b.bottom) - Math.max(a.top, b.top));

  // Content sticking out of its panel, unless a scroll or clip container sits in between.
  for (const panel of document.querySelectorAll(".pnl")) {
    if (!shown(panel)) continue;
    const box = panel.getBoundingClientRect();
    for (const el of panel.querySelectorAll("*")) {
      let clipped = false;
      for (let p = el.parentElement; p && p !== panel; p = p.parentElement) {
        if (getComputedStyle(p).overflowX !== "visible") {
          clipped = true;
          break;
        }
      }
      if (clipped || !shown(el)) continue;
      const r = el.getBoundingClientRect();
      if (r.right > box.right + 1 || r.left < box.left - 1) {
        problems.push(`keluar dari panel: ${label(el)}`);
        break;
      }
    }
  }

  // Buttons drawn on top of each other, and chart labels that were left overlapping.
  const groups = [
    { what: "tombol bertumpuk", items: [...document.querySelectorAll("button, .btn")] },
    { what: "label grafik bertumpuk", items: [...document.querySelectorAll("[data-priority]")] },
  ];
  for (const { what, items } of groups) {
    const list = items.filter(shown);
    for (let i = 0; i < list.length; i++) {
      for (let j = i + 1; j < list.length; j++) {
        const [a, b] = [list[i], list[j]];
        if (a.contains(b) || b.contains(a)) continue;
        if (overlap(a.getBoundingClientRect(), b.getBoundingClientRect()) > 4) problems.push(`${what}: ${label(a)} dan ${label(b)}`);
      }
    }
  }
  return problems;
}
