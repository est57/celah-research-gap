import { Fragment } from "react";

/** Renders an already translated sentence, wrapping each "{name}" placeholder value in a .num span. */
export function NumText({ text, vars }: { text: string; vars: Record<string, string | number> }) {
  return (
    <>
      {text.split(/(\{\w+\})/).map((part, i) => {
        const key = /^\{(\w+)\}$/.exec(part)?.[1];
        return key && key in vars
          ? <span key={i} className="num">{vars[key]}</span>
          : <Fragment key={i}>{part}</Fragment>;
      })}
    </>
  );
}
