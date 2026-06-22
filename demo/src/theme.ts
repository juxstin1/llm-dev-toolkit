// GitHub-dark-inspired palette. Keys double as the color tokens used in script.ts.
export const palette = {
  fg: "#c9d1d9", // default foreground
  bright: "#f0f6fc", // bright white (typed commands)
  dim: "#768390", // comments / labels
  blue: "#54aeff", // directories (tk's ANSI blue)
  magenta: "#d2a8ff", // search paths (tk's ANSI magenta)
  yellow: "#e3b341", // line numbers / sizes (tk's ANSI yellow)
  red: "#ff7b72", // match highlight (tk's ANSI red)
  green: "#7ee787", // prompt user / accents
  cyan: "#39c5cf", // prompt arrow
} as const;

export type Color = keyof typeof palette;

// Window + backdrop
export const ui = {
  bg: "#0d1117", // terminal background
  titlebar: "#161b22", // window titlebar
  border: "#30363d",
  backdropFrom: "#1b2430",
  backdropTo: "#0a0c10",
  trafficRed: "#ff5f57",
  trafficYellow: "#febc2e",
  trafficGreen: "#28c840",
};

export const font = {
  family: "Hack",
  // Sized so the widest demo line (the `tk search` matches, ~87 chars) fits
  // the ~1268px terminal body without clipping at the right edge.
  size: 22, // px
  lineHeight: 33,
};
