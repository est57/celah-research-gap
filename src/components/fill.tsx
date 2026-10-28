import { Fragment, type ReactNode } from "react";

/**
 * Puts React nodes into the "{name}" slots of a sentence that already went through t(),
 * so a sentence with inline markup stays one translation key.
 */
export function fill(text: string, slots: Record<string, ReactNode>): ReactNode {
  return text.split(/(\{\w+\})/).map((part, i) => {
    const name = /^\{(\w+)\}$/.exec(part)?.[1];
    return <Fragment key={i}>{name !== undefined && name in slots ? slots[name] : part}</Fragment>;
  });
}
