import { Step, plainCmd } from "./script";

export const FPS = 30;

// Per-step pacing (frames @ 30fps)
export const PER_CHAR = 2; // typing speed (~15 chars/sec)
export const PAUSE_AFTER_CMD = 14; // beat between Enter and output
export const REVEAL_STAGGER = 2; // delay between output lines
export const REVEAL_FADE = 7; // fade-in length per output line
export const PAUSE_AFTER_OUT = 34; // hold before the next command

export const INTRO = 30; // terminal settles before first keystroke
export const OUTRO = 96; // end-card hold

export type Sched = {
  typeStart: number;
  typeEnd: number;
  outStart: number;
  outEnd: number;
  end: number;
};

export function buildSchedule(steps: Step[]): Sched[] {
  const out: Sched[] = [];
  let t = INTRO;
  for (const step of steps) {
    const typeStart = t;
    const typeEnd = typeStart + plainCmd(step.cmd).length * PER_CHAR;
    const outStart = typeEnd + PAUSE_AFTER_CMD;
    const outEnd = outStart + Math.max(0, step.out.length - 1) * REVEAL_STAGGER + REVEAL_FADE;
    const end = outEnd + PAUSE_AFTER_OUT;
    out.push({ typeStart, typeEnd, outStart, outEnd, end });
    t = end;
  }
  return out;
}

export function totalFrames(steps: Step[]): number {
  const sched = buildSchedule(steps);
  return sched[sched.length - 1].end + OUTRO;
}
