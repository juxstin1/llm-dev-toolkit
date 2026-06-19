import React from "react";
import { Composition } from "remotion";
import { Demo } from "./Demo";
import { steps } from "./script";
import { FPS, totalFrames } from "./timing";

export const RemotionRoot: React.FC = () => {
  return (
    <Composition
      id="TkDemo"
      component={Demo}
      durationInFrames={totalFrames(steps)}
      fps={FPS}
      width={1480}
      height={860}
    />
  );
};
