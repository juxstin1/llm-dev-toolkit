import React from "react";
import { AbsoluteFill, interpolate, spring, useCurrentFrame, useVideoConfig } from "remotion";
import { Terminal } from "./Terminal";
import { steps } from "./script";
import { OUTRO, totalFrames } from "./timing";
import { font, palette } from "./theme";

const Outro: React.FC<{ start: number }> = ({ start }) => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();
  const local = frame - start;
  const fade = interpolate(local, [0, 22], [0, 1], { extrapolateRight: "clamp" });
  const pop = spring({ frame: local, fps, config: { damping: 14, mass: 0.6 } });

  return (
    <AbsoluteFill
      style={{
        background: `rgba(5,7,10,${0.94 * fade})`,
        justifyContent: "center",
        alignItems: "center",
        fontFamily: font.family,
      }}
    >
      <div style={{ textAlign: "center", opacity: fade, transform: `scale(${0.92 + pop * 0.08})` }}>
        <div style={{ fontSize: 132, fontWeight: 700, color: palette.bright, letterSpacing: 2 }}>
          <span style={{ color: palette.cyan }}>tk</span>
        </div>
        <div style={{ fontSize: 40, color: palette.fg, marginTop: 4 }}>LLM Dev Toolkit</div>
        <div style={{ fontSize: 26, color: palette.dim, marginTop: 26 }}>
          a fast, git-aware file toolkit · single binary
        </div>
        <div style={{ fontSize: 26, color: palette.green, marginTop: 14 }}>
          github.com/juxstin1/llm-dev-toolkit
        </div>
      </div>
    </AbsoluteFill>
  );
};

export const Demo: React.FC = () => {
  const frame = useCurrentFrame();
  const outroStart = totalFrames(steps) - OUTRO;
  return (
    <>
      <Terminal />
      {frame >= outroStart ? <Outro start={outroStart} /> : null}
    </>
  );
};
