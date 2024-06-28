/**
 * DB Knob
 *
 *
 *
 * Source:
 * https://github.com/satelllte/react-knob-headless/blob/main/apps/docs/src/components/knobs/KnobFrequency.tsx
 */

import { NormalisableRange, dbToGain, gainToDb } from "../../lib/utils";
import { KnobBase } from "./KnobBase";

type KnobBaseProps = React.ComponentProps<typeof KnobBase>;
type KnobFrequencyProps = Pick<
  KnobBaseProps,
  "theme" | "label" | "orientation" | "rawGain" | "setRawGain"
>;

export function DBKnob(props: KnobFrequencyProps) {
  //
  const valueMin = dbToGain(-30);
  const valueMax = dbToGain(30);
  const valueDefault = dbToGain(0);
  // step functions are for keyboard control
  const stepFn = (valueRaw: number): number => 0;
  const stepLargerFn = (valueRaw: number): number => 0;
  //
  const normalisableRange = new NormalisableRange(
    valueMin,
    valueMax,
    valueDefault
  );
  //
  const mapTo01 = (x: number) => normalisableRange.mapTo01(x);
  const mapFrom01 = (x: number) => normalisableRange.mapFrom01(x);

  const valueRawRoundFn = (x: number): number => Number(x.toFixed(2));
  const valueRawDisplayFn = (valueRaw: number): string =>
    `${valueRawRoundFn(gainToDb(valueRaw))} dB`;

  return (
    <KnobBase
      valueDefault={valueDefault}
      valueMin={valueMin}
      valueMax={valueMax}
      stepFn={stepFn}
      stepLargerFn={stepLargerFn}
      valueRawRoundFn={valueRawRoundFn}
      valueRawDisplayFn={valueRawDisplayFn}
      mapTo01={mapTo01}
      mapFrom01={mapFrom01}
      {...props}
    />
  );
}
