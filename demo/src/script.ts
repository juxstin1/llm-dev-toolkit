import { Color } from "./theme";

export type Seg = { t: string; c?: Color; b?: boolean };
export type Line = Seg[];
export type Step = { cmd: Seg[]; out: Line[] };

const s = (t: string, c?: Color, b?: boolean): Seg => ({ t, c, b });

// `tk <args>` with the binary name emphasized.
const cmd = (args: string): Seg[] => [s("tk", "bright", true), s(" " + args, "bright")];

// ---- tk ll -------------------------------------------------------------
type Row = { perms: string; size: string; date: string; name: string; dir: boolean };
const lsRows: Row[] = [
  { perms: "drwxr-xr-x", size: "-", date: "2026-06-19 11:30", name: ".github", dir: true },
  { perms: "drwxr-xr-x", size: "-", date: "2026-06-19 11:38", name: "src", dir: true },
  { perms: "drwxr-xr-x", size: "-", date: "2026-06-19 11:39", name: "tests", dir: true },
  { perms: "-rw-r--r--", size: "196 B", date: "2026-06-19 11:31", name: ".gitattributes", dir: false },
  { perms: "-rw-r--r--", size: "8 B", date: "2026-06-19 09:32", name: ".gitignore", dir: false },
  { perms: "-rw-r--r--", size: "47.72 KiB", date: "2026-06-19 10:26", name: "Cargo.lock", dir: false },
  { perms: "-rw-r--r--", size: "810 B", date: "2026-06-19 11:30", name: "Cargo.toml", dir: false },
  { perms: "-rw-r--r--", size: "11.08 KiB", date: "2026-06-19 11:30", name: "LICENSE", dir: false },
  { perms: "-rw-r--r--", size: "4.55 KiB", date: "2026-06-19 11:39", name: "README.md", dir: false },
];
const lsOut: Line[] = lsRows.map((r) => [
  s(r.perms + " ", "dim"),
  s(r.size.padStart(9) + "  ", "fg"),
  s(r.date + "  ", "dim"),
  r.dir ? s(r.name + "/", "blue", true) : s(r.name, "fg"),
]);

// ---- tk tree -L 2 ------------------------------------------------------
type TreeRow = { branch: string; name: string; dir: boolean };
const treeRows: TreeRow[] = [
  { branch: "", name: ".", dir: false },
  { branch: "├── ", name: "Cargo.lock", dir: false },
  { branch: "├── ", name: "Cargo.toml", dir: false },
  { branch: "├── ", name: "LICENSE", dir: false },
  { branch: "├── ", name: "README.md", dir: false },
  { branch: "├── ", name: "src", dir: true },
  { branch: "│   ├── ", name: "commands", dir: true },
  { branch: "│   └── ", name: "main.rs", dir: false },
  { branch: "└── ", name: "tests", dir: true },
  { branch: "    └── ", name: "cli.rs", dir: false },
];
const treeOut: Line[] = treeRows.map((r) => [
  s(r.branch, "dim"),
  s(r.name, r.dir ? "blue" : "fg", r.dir),
]);

// ---- tk search "pub fn run" --line-number ------------------------------
type Hit = { path: string; line: string; tail: string };
const hits: Hit[] = [
  { path: "src/commands/checksum.rs", line: "57", tail: "(args: &ChecksumArgs) -> Result<(), String> {" },
  { path: "src/commands/clip.rs", line: "43", tail: "(args: &crate::ClipArgs) -> Result<(), String> {" },
  { path: "src/commands/count.rs", line: "87", tail: "(args: &CountArgs) -> Result<(), String> {" },
  { path: "src/commands/dups.rs", line: "159", tail: "(args: &DupsArgs) -> Result<(), String> {" },
  { path: "src/commands/extract.rs", line: "4", tail: "(args: &crate::ExtractArgs) -> Result<(), String> {" },
  { path: "src/commands/json.rs", line: "16", tail: "(args: &crate::JsonArgs) -> Result<(), String> {" },
  { path: "src/commands/largest.rs", line: "7", tail: "(args: &LargestArgs) -> Result<(), String> {" },
];
const searchOut: Line[] = hits.map((h) => [
  s(h.path, "magenta"),
  s(":", "dim"),
  s(h.line, "yellow"),
  s(":", "dim"),
  s("pub fn run", "red"),
  s(h.tail, "fg"),
]);

// ---- tk stats ----------------------------------------------------------
const statsOut: Line[] = [
  [s("  Files:   ", "dim"), s("27".padStart(11), "fg")],
  [s("  Dirs:    ", "dim"), s("7".padStart(11), "fg")],
  [s("  Total:   ", "dim"), s("159.08 KiB".padStart(11), "green")],
];

// ---- tk largest -n 5 ---------------------------------------------------
type Big = { size: string; name: string };
const bigs: Big[] = [
  { size: "47.72 KiB", name: "Cargo.lock" },
  { size: "11.91 KiB", name: "src/main.rs" },
  { size: "11.08 KiB", name: "LICENSE" },
  { size: "8.99 KiB", name: "tests/cli.rs" },
  { size: "7.98 KiB", name: "src/commands/dups.rs" },
];
const largestOut: Line[] = bigs.map((b, i) => [
  s(`${i + 1}`.padStart(4) + ".  ", "dim"),
  s(b.size.padStart(9) + "  ", "yellow"),
  s(b.name, "fg"),
]);

export const steps: Step[] = [
  { cmd: cmd("ll"), out: lsOut },
  { cmd: cmd("tree -L 2"), out: treeOut },
  { cmd: cmd('search "pub fn run" --line-number'), out: searchOut },
  { cmd: cmd("stats"), out: statsOut },
  { cmd: cmd("largest -n 5"), out: largestOut },
];

export const plainCmd = (segs: Seg[]): string => segs.map((x) => x.t).join("");
