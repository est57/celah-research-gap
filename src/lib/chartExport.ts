import { t } from "../i18n";
// Chart images: the on-screen SVG cloned with its computed styles inlined, always in
// the light theme on a white background, as standalone SVG or a 2x PNG.

const PROPS = [
  "fill", "fill-opacity", "stroke", "stroke-width", "stroke-opacity", "stroke-dasharray", "stroke-linejoin",
  "stroke-linecap", "opacity", "font-family", "font-size", "font-weight", "font-style", "text-anchor",
  "dominant-baseline", "paint-order", "visibility",
];
const NS = "http://www.w3.org/2000/svg";

export interface SvgImage {
  text: string;
  width: number;
  height: number;
}

/** Serializes `svg` with light-theme colors. Styles go through CSSOM, which the CSP allows. */
export function serializeSvg(svg: SVGSVGElement): SvgImage {
  const root = document.documentElement;
  const theme = root.dataset.theme;
  root.dataset.theme = "light";
  try {
    const clone = svg.cloneNode(true) as SVGSVGElement;
    const from = [svg, ...svg.querySelectorAll<SVGElement>("*")];
    const to = [clone, ...clone.querySelectorAll<SVGElement>("*")];
    from.forEach((el, i) => {
      const computed = getComputedStyle(el);
      const target = to[i];
      target.removeAttribute("class");
      target.removeAttribute("style");
      for (const p of PROPS) target.style.setProperty(p, computed.getPropertyValue(p));
    });
    const { width, height } = svg.getBoundingClientRect();
    clone.setAttribute("xmlns", NS);
    clone.setAttribute("width", String(Math.round(width)));
    clone.setAttribute("height", String(Math.round(height)));
    clone.setAttribute("viewBox", `0 0 ${Math.round(width)} ${Math.round(height)}`);
    const bg = document.createElementNS(NS, "rect");
    bg.setAttribute("width", "100%");
    bg.setAttribute("height", "100%");
    bg.style.setProperty("fill", "#ffffff");
    clone.insertBefore(bg, clone.firstChild);
    return { text: new XMLSerializer().serializeToString(clone), width: Math.round(width), height: Math.round(height) };
  } finally {
    if (theme === undefined) delete root.dataset.theme;
    else root.dataset.theme = theme;
  }
}

/** Renders the SVG at `scale` times its size and returns PNG bytes. */
export async function svgToPng(image: SvgImage, scale = 2): Promise<Uint8Array> {
  const img = new Image();
  await new Promise<void>((resolve, reject) => {
    img.onload = () => resolve();
    img.onerror = () => reject(new Error(t("Grafik tidak bisa diubah menjadi PNG.")));
    img.src = `data:image/svg+xml;charset=utf-8,${encodeURIComponent(image.text)}`;
  });
  const canvas = document.createElement("canvas");
  canvas.width = image.width * scale;
  canvas.height = image.height * scale;
  const ctx = canvas.getContext("2d");
  if (!ctx) throw new Error(t("Kanvas tidak tersedia."));
  ctx.scale(scale, scale);
  ctx.drawImage(img, 0, 0);
  const blob = await new Promise<Blob | null>((resolve) => canvas.toBlob(resolve, "image/png"));
  if (!blob) throw new Error(t("PNG tidak bisa dibuat."));
  return new Uint8Array(await blob.arrayBuffer());
}
