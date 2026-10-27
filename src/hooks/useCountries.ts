import { countries } from "../lib/api";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleCountries").then((m) => m.SAMPLE_COUNTRIES) : null;

export function useCountries(version: number) {
  return useDesktopData(countries, sample, version);
}
