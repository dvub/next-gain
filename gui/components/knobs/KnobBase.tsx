/**
 * Modified knob BASE -
 * source:
 * https://github.com/satelllte/react-knob-headless/blob/main/apps/docs/src/components/knobs/KnobBase.tsx
 */

import clsx from "clsx";
import { useId, useState } from "react";
import {
  KnobHeadless,
  KnobHeadlessLabel,
  KnobHeadlessOutput,
  useKnobKeyboardControls,
} from "react-knob-headless";
import { mapFrom01Linear, mapTo01Linear } from "@dsp-ts/math";
import { KnobBaseThumb } from "./KnobBaseThumb";
import { sendToPlugin } from "../../lib";

type KnobHeadlessProps = React.ComponentProps<typeof KnobHeadless>;
type KnobBaseThumbProps = React.ComponentProps<typeof KnobBaseThumb>;
type KnobBaseProps = Pick<
  KnobHeadlessProps,
  | "valueMin"
  | "valueMax"
  | "valueRawRoundFn"
  | "valueRawDisplayFn"
  | "orientation"
  | "mapTo01"
  | "mapFrom01"
> &
  Pick<KnobBaseThumbProps, "theme"> & {
    readonly label: string;
    readonly valueDefault: number;
    readonly stepFn: (valueRaw: number) => number;
    readonly stepLargerFn: (valueRaw: number) => number;
    rawGain: number;
    setRawGain: React.Dispatch<React.SetStateAction<number>>;
  };

export function KnobBase({
  theme,
  label,
  valueDefault,
  valueMin,
  valueMax,
  valueRawRoundFn,
  valueRawDisplayFn,
  orientation,
  stepFn,
  stepLargerFn,
  rawGain,
  setRawGain,
  mapTo01 = mapTo01Linear,
  mapFrom01 = mapFrom01Linear,
}: KnobBaseProps) {
  const knobId = useId();
  const labelId = useId();
  const value01 = mapTo01(rawGain, valueMin, valueMax);
  const step = stepFn(rawGain);
  const stepLarger = stepLargerFn(rawGain);
  const dragSensitivity = 0.003;
  const keyboardControlHandlers = useKnobKeyboardControls({
    valueRaw: rawGain,
    valueMin,
    valueMax,
    step,
    stepLarger,
    onValueRawChange: setVal,
  });

  // in addition to changing the state,
  // we want to also send a message to the plugin backend here
  function setVal(valueRaw: number) {
    setRawGain(valueRaw);
    sendToPlugin({ type: "SetGain", value: valueRaw });
  }

  return (
    <div
      className={clsx(
        "w-24 flex flex-col gap-0.5 justify-center items-center text-xs select-none",
        "outline-none"
      )}
    >
      <KnobHeadlessLabel id={labelId}>{label}</KnobHeadlessLabel>
      <KnobHeadless
        id={knobId}
        aria-labelledby={labelId}
        className="relative w-24 h-24 outline-none"
        valueMin={valueMin}
        valueMax={valueMax}
        valueRaw={rawGain}
        valueRawRoundFn={valueRawRoundFn}
        valueRawDisplayFn={valueRawDisplayFn}
        dragSensitivity={dragSensitivity}
        orientation={orientation}
        mapTo01={mapTo01}
        mapFrom01={mapFrom01}
        onValueRawChange={setVal}
        {...keyboardControlHandlers}
      >
        <KnobBaseThumb theme={theme} value01={value01} />
      </KnobHeadless>
      <KnobHeadlessOutput htmlFor={knobId}>
        {valueRawDisplayFn(rawGain)}
      </KnobHeadlessOutput>
    </div>
  );
}
