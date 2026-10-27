import { authorTable, sourcesDetail } from "../lib/api";
import { affiliations, countryTable } from "../lib/apiEntities";
import { useDesktopData } from "./useDesktopData";

// The DEV check sits on the import itself so production builds drop the sample module entirely.
const sample = import.meta.env.DEV ? () => import("../dev/sampleEntities") : null;

export function useSourcesDetail(version: number) {
  return useDesktopData(sourcesDetail, sample ? () => sample().then((m) => m.SAMPLE_SOURCES) : null, version);
}

export function useAuthorTable(version: number) {
  return useDesktopData(authorTable, sample ? () => sample().then((m) => m.SAMPLE_AUTHORS) : null, version);
}

export function useAffiliations(version: number) {
  return useDesktopData(affiliations, sample ? () => sample().then((m) => m.SAMPLE_AFFILIATIONS) : null, version);
}

export function useCountryTable(version: number) {
  return useDesktopData(countryTable, sample ? () => sample().then((m) => m.SAMPLE_COUNTRY_TABLE) : null, version);
}
