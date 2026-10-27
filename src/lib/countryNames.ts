let display: Intl.DisplayNames | null = null;
try {
  display = new Intl.DisplayNames(["id"], { type: "region" });
} catch {
  display = null;
}

/** Indonesian country name for an ISO alpha-2 code ("CN" -> "Tiongkok"), or the code itself. */
export function countryName(code: string): string {
  try {
    return display?.of(code) ?? code;
  } catch {
    return code;
  }
}
