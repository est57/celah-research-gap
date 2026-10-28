// English dictionary, one folder per area of the app so each stays small.
import { DICT as shell } from "./shell";
import { DICT as collection } from "./collection";
import { DICT as sources } from "./sources";
import { DICT as conceptual } from "./conceptual";
import { DICT as structure } from "./structure";
import { DICT as gaps } from "./gaps";

export const EN: Readonly<Record<string, string>> = { ...shell, ...collection, ...sources, ...conceptual, ...structure, ...gaps };
