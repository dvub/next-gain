"use client";

import { PluginMessage } from "@/bindings/PluginMessage";
import { DBKnob } from "@/components/knobs/DBKnob";
import { gainToDb } from "@/lib/utils";
import clsx from "clsx";
import { useState, useEffect } from "react";

export default function Home() {
  const [rawGain, setRawGain] = useState(0);
  const [peak, setPeak] = useState(-100);
  const [max, setMax] = useState(-100);
  useEffect(() => {
    window.onPluginMessage = (msg: PluginMessage) => {
      switch (msg.type) {
        case "ParamChange": {
          setRawGain(msg.value);
          break;
        }
        case "PeakMeterData": {
          let db = Math.max(-100, gainToDb(msg.value));

          setPeak(db);
          break;
        }
      }
    };
  }, []);

  useEffect(() => {
    if (peak > max) {
      setMax(peak);
    }
  }, [peak]);

  return (
    <div className="w-screen h-screen bg-[#2f1861] text-white">
      <div className="text-center flex justify-center">
        <div className="py-5">
          <h1 className="font-extrabold text-4xl">Gainify</h1>
          <p>A proof of concept for web GUI</p>
        </div>
      </div>
      {/**
       * the star of the show...
       * the gain knob!!
       */}
      <div className="px-10 w-full flex justify-between">
        <div className="relative">
          <DBKnob
            label="Gain"
            theme="pink"
            rawGain={rawGain}
            setRawGain={setRawGain}
          />
        </div>
        <div className="relative w-24 flex justify-center text-center text-xs">
          <div className="w-full">
            <p className="pb-3">Output</p>
            <div className="relative flex justify-center h-[100px]">
              <div
                style={{ height: `${100 + peak}px` }}
                className="absolute bottom-0 w-[25px] bg-gradient-to-t from-green-500 to-green-400 border-t-2 border-black"
              />
            </div>
            <div>
              <p>{peak.toFixed(2)} dB</p>
              <p>Peak: {max.toFixed(2)} dB</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
