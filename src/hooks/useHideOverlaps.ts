import { useLayoutEffect, type DependencyList, type RefObject } from "react";

const overlaps = (a: DOMRect, b: DOMRect) => a.left < b.right && b.left < a.right && a.top < b.bottom && b.top < a.bottom;

/**
 * Hides labels (matched by `selector` inside `ref`) that would cover a more important one.
 * Importance comes from each label's `data-priority`; measuring the rendered boxes keeps it
 * exact for any font size. Runs again when the container resizes.
 */
export function useHideOverlaps(ref: RefObject<HTMLElement | null>, selector: string, deps: DependencyList) {
  useLayoutEffect(() => {
    const root = ref.current;
    if (!root) return;
    const run = () => {
      const labels = [...root.querySelectorAll<HTMLElement>(selector)];
      labels.forEach((l) => (l.style.visibility = ""));
      labels.sort((a, b) => Number(b.dataset.priority) - Number(a.dataset.priority));
      const kept: DOMRect[] = [];
      for (const l of labels) {
        const r = l.getBoundingClientRect();
        if (kept.some((k) => overlaps(k, r))) l.style.visibility = "hidden";
        else kept.push(r);
      }
    };
    run();
    const observer = new ResizeObserver(run);
    observer.observe(root);
    return () => observer.disconnect();
  }, deps);
}
