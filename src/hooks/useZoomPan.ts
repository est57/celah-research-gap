import { useCallback, useEffect, useRef, useState, type PointerEvent } from "react";

export interface View {
  k: number;
  tx: number;
  ty: number;
}

const MIN = 1;
const MAX = 8;
const IDENTITY: View = { k: 1, tx: 0, ty: 0 };

/** Keeps the zoomed content covering the plot, so it can never be panned out of sight. */
function clamp(v: View, w: number, h: number): View {
  const k = Math.min(MAX, Math.max(MIN, v.k));
  return { k, tx: Math.min(0, Math.max(w - w * k, v.tx)), ty: Math.min(0, Math.max(h - h * k, v.ty)) };
}

/**
 * Wheel zoom around the pointer, drag to pan and button zoom for an SVG of `width` x `height`.
 * `moved` tells click handlers that the pointer was dragging, so a pan does not also select.
 */
export function useZoomPan(width: number, height: number) {
  const ref = useRef<SVGSVGElement>(null);
  const [view, setView] = useState<View>(IDENTITY);
  const drag = useRef<{ x: number; y: number; start: View; moved: boolean } | null>(null);
  const moved = useRef(false);

  const zoomAt = useCallback((factor: number, px: number, py: number) => {
    setView((v) => {
      const k = Math.min(MAX, Math.max(MIN, v.k * factor));
      return clamp({ k, tx: px - (px - v.tx) * (k / v.k), ty: py - (py - v.ty) * (k / v.k) }, width, height);
    });
  }, [width, height]);

  useEffect(() => {
    const el = ref.current;
    if (!el) return;
    // A native listener, because React wheel handlers are passive and cannot stop the page scrolling.
    const onWheel = (e: WheelEvent) => {
      e.preventDefault();
      const box = el.getBoundingClientRect();
      zoomAt(Math.exp(-e.deltaY * 0.0015), e.clientX - box.left, e.clientY - box.top);
    };
    el.addEventListener("wheel", onWheel, { passive: false });
    return () => el.removeEventListener("wheel", onWheel);
  }, [zoomAt]);

  useEffect(() => setView((v) => clamp(v, width, height)), [width, height]);

  const handlers = {
    onPointerDown: (e: PointerEvent<SVGSVGElement>) => {
      moved.current = false;
      drag.current = { x: e.clientX, y: e.clientY, start: view, moved: false };
    },
    onPointerMove: (e: PointerEvent<SVGSVGElement>) => {
      const d = drag.current;
      if (!d) return;
      const dx = e.clientX - d.x, dy = e.clientY - d.y;
      if (!d.moved && Math.hypot(dx, dy) < 4) return;
      if (!d.moved) e.currentTarget.setPointerCapture(e.pointerId);
      d.moved = true;
      moved.current = true;
      setView(clamp({ k: d.start.k, tx: d.start.tx + dx, ty: d.start.ty + dy }, width, height));
    },
    onPointerUp: () => { drag.current = null; },
  };

  return {
    ref,
    view,
    handlers,
    /** True right after a drag, until the next pointer press. */
    wasDrag: () => moved.current,
    zoomIn: () => zoomAt(1.5, width / 2, height / 2),
    zoomOut: () => zoomAt(1 / 1.5, width / 2, height / 2),
    reset: () => setView(IDENTITY),
  };
}
