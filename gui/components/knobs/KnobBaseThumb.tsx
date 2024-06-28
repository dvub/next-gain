/**
 * Modified knob thumb -
 * original source:
 * https://github.com/satelllte/react-knob-headless/blob/main/apps/docs/src/components/knobs/KnobBaseThumb.tsx
 */
import clsx from "clsx";
import { mapFrom01Linear } from "@dsp-ts/math";

type KnobBaseThumbProps = {
  readonly theme: "stone" | "pink" | "green" | "sky";
  readonly value01: number;
};

export function KnobBaseThumb({ theme, value01 }: KnobBaseThumbProps) {
  const angleMin = -145;
  const angleMax = 145;
  const angle = mapFrom01Linear(value01, angleMin, angleMax);
  return (
    <div
      className={clsx(
        "absolute h-full w-full rounded-full",

        theme === "stone" && "bg-stone-300",
        theme === "pink" && "bg-pink-300",
        theme === "green" && "bg-green-300",
        theme === "sky" && "bg-sky-300"
      )}
    >
      {/* Pointer line thingy - is it called a thumb ?? */}
      <div className="absolute h-full w-full" style={{ rotate: `${angle}deg` }}>
        <div className="absolute left-1/2 top-0 h-1/2 w-[2px] -translate-x-1/2 rounded-sm bg-slate-950" />
      </div>
    </div>
  );
}
