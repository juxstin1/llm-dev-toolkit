import React, { useEffect, useState } from "react";
import {
  AbsoluteFill,
  continueRender,
  delayRender,
  interpolate,
  staticFile,
  useCurrentFrame,
} from "remotion";
import { font, palette, ui } from "./theme";
import { Seg, plainCmd, steps } from "./script";
import { buildSchedule } from "./timing";

const WINDOW_W = 1320;
const WINDOW_H = 720;
const TITLEBAR_H = 48;
const BODY_PAD = 26;
const BLINK = 15;

const useHackFont = () => {
  const [handle] = useState(() => delayRender("Loading Hack font"));
  useEffect(() => {
    const reg = new FontFace("Hack", `url(${staticFile("hack-regular.woff2")}) format("woff2")`, {
      weight: "400",
    });
    const bold = new FontFace("Hack", `url(${staticFile("hack-bold.woff2")}) format("woff2")`, {
      weight: "700",
    });
    Promise.all([reg.load(), bold.load()])
      .then(([r, b]) => {
        document.fonts.add(r);
        document.fonts.add(b);
        continueRender(handle);
      })
      .catch(() => continueRender(handle));
  }, [handle]);
};

const Span: React.FC<{ seg: Seg }> = ({ seg }) => (
  <span style={{ color: seg.c ? palette[seg.c] : palette.fg, fontWeight: seg.b ? 700 : 400 }}>
    {seg.t}
  </span>
);

const Prompt: React.FC = () => (
  <>
    <span style={{ color: palette.green, fontWeight: 700 }}>justin</span>
    <span style={{ color: palette.dim }}> </span>
    <span style={{ color: palette.blue }}>~/llm-dev-toolkit</span>
    <span style={{ color: palette.dim }}> </span>
    <span style={{ color: palette.magenta }}>git:(</span>
    <span style={{ color: palette.red }}>main</span>
    <span style={{ color: palette.magenta }}>)</span>
    <span style={{ color: palette.cyan }}> ❯ </span>
  </>
);

const Cursor: React.FC<{ on: boolean }> = ({ on }) => (
  <span
    style={{
      display: "inline-block",
      width: "0.6em",
      height: "1.05em",
      transform: "translateY(0.18em)",
      background: on ? palette.fg : "transparent",
      borderRadius: 1,
    }}
  />
);

// First `n` characters across a list of segments.
const sliceSegs = (segs: Seg[], n: number): Seg[] => {
  const out: Seg[] = [];
  let left = n;
  for (const seg of segs) {
    if (left <= 0) break;
    if (seg.t.length <= left) {
      out.push(seg);
      left -= seg.t.length;
    } else {
      out.push({ ...seg, t: seg.t.slice(0, left) });
      left = 0;
    }
  }
  return out;
};

export const Terminal: React.FC = () => {
  useHackFont();
  const frame = useCurrentFrame();
  const sched = buildSchedule(steps);
  const blinkOn = Math.floor(frame / BLINK) % 2 === 0;

  type RowEl = { key: string; el: React.ReactNode };
  const rows: RowEl[] = [];

  const lineStyle: React.CSSProperties = { height: font.lineHeight, whiteSpace: "pre" };

  steps.forEach((step, i) => {
    const sc = sched[i];
    if (frame < sc.typeStart) return;

    // Command line (with prompt)
    const typing = frame < sc.typeEnd;
    const inCmdPhase = frame < sc.outStart;
    const fullLen = plainCmd(step.cmd).length;
    const shown = typing
      ? Math.min(fullLen, Math.floor((frame - sc.typeStart) / 2))
      : fullLen;
    const cmdSegs = sliceSegs(step.cmd, shown);
    const showCursor = inCmdPhase && (typing ? true : blinkOn);

    rows.push({
      key: `cmd-${i}`,
      el: (
        <div style={lineStyle}>
          <Prompt />
          {cmdSegs.map((seg, j) => (
            <Span key={j} seg={seg} />
          ))}
          {showCursor ? <Cursor on /> : null}
        </div>
      ),
    });

    // Output lines (revealed after Enter)
    if (frame >= sc.outStart) {
      step.out.forEach((line, j) => {
        const lineStart = sc.outStart + j * 2;
        if (frame < lineStart) return;
        const opacity = interpolate(frame, [lineStart, lineStart + 7], [0, 1], {
          extrapolateRight: "clamp",
        });
        rows.push({
          key: `out-${i}-${j}`,
          el: (
            <div style={{ ...lineStyle, opacity }}>
              {line.map((seg, k) => (
                <Span key={k} seg={seg} />
              ))}
            </div>
          ),
        });
      });
    }
  });

  // Trailing prompt with a blinking cursor while waiting for the next command.
  const active = [...sched].reverse().find((sc) => frame >= sc.typeStart);
  const nextNotStarted = !sched.some((sc) => frame >= sc.typeStart && frame < sc.outStart);
  if (active && frame >= active.outEnd && nextNotStarted) {
    rows.push({
      key: "trailing",
      el: (
        <div style={lineStyle}>
          <Prompt />
          {blinkOn ? <Cursor on /> : null}
        </div>
      ),
    });
  }

  // Auto-scroll: keep the newest rows in view.
  const bodyInner = WINDOW_H - TITLEBAR_H - BODY_PAD * 2;
  const contentH = rows.length * font.lineHeight;
  const translateY = Math.min(0, bodyInner - contentH);

  const windowOpacity = interpolate(frame, [0, 18], [0, 1], { extrapolateRight: "clamp" });
  const windowLift = interpolate(frame, [0, 18], [18, 0], { extrapolateRight: "clamp" });

  return (
    <AbsoluteFill
      style={{
        background: `radial-gradient(circle at 50% 30%, ${ui.backdropFrom}, ${ui.backdropTo})`,
        justifyContent: "center",
        alignItems: "center",
        fontFamily: font.family,
      }}
    >
      <div
        style={{
          width: WINDOW_W,
          height: WINDOW_H,
          background: ui.bg,
          borderRadius: 12,
          border: `1px solid ${ui.border}`,
          boxShadow: "0 30px 80px rgba(0,0,0,0.55)",
          overflow: "hidden",
          opacity: windowOpacity,
          transform: `translateY(${windowLift}px)`,
        }}
      >
        {/* Titlebar */}
        <div
          style={{
            height: TITLEBAR_H,
            background: ui.titlebar,
            borderBottom: `1px solid ${ui.border}`,
            display: "flex",
            alignItems: "center",
            paddingLeft: 18,
            position: "relative",
          }}
        >
          <div style={{ display: "flex", gap: 9 }}>
            {[ui.trafficRed, ui.trafficYellow, ui.trafficGreen].map((c) => (
              <div key={c} style={{ width: 14, height: 14, borderRadius: "50%", background: c }} />
            ))}
          </div>
          <div
            style={{
              position: "absolute",
              left: 0,
              right: 0,
              textAlign: "center",
              color: palette.dim,
              fontSize: 18,
            }}
          >
            justin@dev — llm-dev-toolkit — zsh
          </div>
        </div>

        {/* Body */}
        <div style={{ height: WINDOW_H - TITLEBAR_H, padding: BODY_PAD, overflow: "hidden" }}>
          <div
            style={{
              fontSize: font.size,
              lineHeight: `${font.lineHeight}px`,
              color: palette.fg,
              transform: `translateY(${translateY}px)`,
            }}
          >
            {rows.map((r) => (
              <React.Fragment key={r.key}>{r.el}</React.Fragment>
            ))}
          </div>
        </div>
      </div>
    </AbsoluteFill>
  );
};
