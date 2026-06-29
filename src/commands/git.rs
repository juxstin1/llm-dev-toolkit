#![allow(
    clippy::manual_strip,
    clippy::needless_splitn,
    clippy::redundant_slicing
)]

use clap::Args;
use serde::Serialize;
use std::process::Command;

// ---------------------------------------------------------------------------
// Args
// ---------------------------------------------------------------------------

#[derive(Args)]
pub struct StatusArgs {
    path: Option<String>,
    #[arg(long, help = "Use raw porcelain format (text mode)")]
    porcelain: bool,
}

#[derive(Args)]
pub struct DiffArgs {
    #[arg(long, help = "Show staged diff instead of unstaged")]
    staged: bool,
    #[arg(long, alias = "cached", help = "Alias for --staged")]
    cached: bool,
    #[arg(short = 'C', long, default_value = "3", help = "Context lines")]
    context: usize,
    paths: Vec<String>,
}

#[derive(Args)]
pub struct LogArgs {
    #[arg(short = 'n', long, default_value = "10", help = "Number of commits")]
    count: usize,
    #[arg(long, help = "Show commits since this date (e.g. '7 days ago')")]
    since: Option<String>,
    #[arg(long, help = "Show commits until this date")]
    until: Option<String>,
    #[arg(long, help = "Filter by author pattern")]
    author: Option<String>,
    paths: Vec<String>,
}

#[derive(Args)]
pub struct BranchArgs {
    #[arg(short = 'a', long, help = "Show all branches (including remote)")]
    all: bool,
}

// ---------------------------------------------------------------------------
// Run
// ---------------------------------------------------------------------------

pub fn run_status(args: &StatusArgs) -> Result<(), String> {
    crate::config::require_feature("git")?;
    ensure_git_repo()?;

    let output = run_git(&["status", "--branch", "--porcelain=v1"])?;

    if crate::commands::json_enabled() {
        let parsed = parse_status(&output, &args.path)?;
        crate::commands::emit_json(&parsed)
    } else {
        // Raw git status output for human reading
        let raw = run_git(&["status"])?;
        print!("{}", raw);
        Ok(())
    }
}

pub fn run_diff(args: &DiffArgs) -> Result<(), String> {
    crate::config::require_feature("git")?;
    ensure_git_repo()?;

    let is_staged = args.staged || args.cached;
    let mut cmd = vec!["diff".to_string()];
    if is_staged {
        cmd.push("--cached".into());
    }
    cmd.push(format!("--unified={}", args.context));
    cmd.push("--".into());
    cmd.extend(args.paths.iter().cloned());

    let cmd_refs: Vec<&str> = cmd.iter().map(|s| s.as_str()).collect();
    let output = run_git(&cmd_refs)?;

    if crate::commands::json_enabled() {
        let parsed = parse_diff(&output)?;
        crate::commands::emit_json(&parsed)
    } else {
        print!("{}", output);
        Ok(())
    }
}

pub fn run_log(args: &LogArgs) -> Result<(), String> {
    crate::config::require_feature("git")?;
    ensure_git_repo()?;

    let mut git_args: Vec<String> = vec![
        "log".into(),
        "--format=%H%n%h%n%an%n%ae%n%ai%n%cn%n%ce%n%cI%n%D%n%s%n%b%x00".into(),
        format!("--max-count={}", args.count),
    ];
    if let Some(ref since) = args.since {
        git_args.push("--since".into());
        git_args.push(since.into());
    }
    if let Some(ref until) = args.until {
        git_args.push("--until".into());
        git_args.push(until.into());
    }
    if let Some(ref author) = args.author {
        git_args.push("--author".into());
        git_args.push(author.into());
    }
    git_args.push("--".into());
    git_args.extend(args.paths.iter().cloned());

    let git_refs: Vec<&str> = git_args.iter().map(|s| s.as_str()).collect();
    let output = run_git(&git_refs)?;

    if crate::commands::json_enabled() {
        let parsed = parse_log(&output)?;
        crate::commands::emit_json(&parsed)
    } else {
        let fmt = "--format=%C(auto)%h %s %C(dim)%an%C(reset)";
        let mut text_args: Vec<String> = vec![
            "log".into(),
            fmt.into(),
            format!("--max-count={}", args.count),
        ];
        if let Some(ref since) = args.since {
            text_args.push("--since".into());
            text_args.push(since.into());
        }
        if let Some(ref until) = args.until {
            text_args.push("--until".into());
            text_args.push(until.into());
        }
        if let Some(ref author) = args.author {
            text_args.push("--author".into());
            text_args.push(author.into());
        }
        text_args.push("--".into());
        text_args.extend(args.paths.iter().cloned());
        let text_refs: Vec<&str> = text_args.iter().map(|s| s.as_str()).collect();
        let out = run_git(&text_refs)?;
        print!("{}", out);
        Ok(())
    }
}

pub fn run_branch(args: &BranchArgs) -> Result<(), String> {
    crate::config::require_feature("git")?;
    ensure_git_repo()?;

    if crate::commands::json_enabled() {
        let list_args = if args.all {
            vec!["branch", "--all"]
        } else {
            vec!["branch"]
        };
        let output = run_git(&list_args)?;
        let parsed = parse_branch_list(&output)?;
        crate::commands::emit_json(&parsed)
    } else {
        let mut cmd_args = vec!["branch"];
        if args.all {
            cmd_args.push("-a");
        }
        let output = run_git(&cmd_args)?;
        print!("{}", output);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn run_git(args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|_| "Failed to run git. Is git installed and on PATH?".to_string())?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(stderr);
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn ensure_git_repo() -> Result<(), String> {
    let output = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
        .map_err(|_| "Failed to run git.".to_string())?;

    if !output.status.success() {
        return Err("Not a git repository (or any of the parent directories)".to_string());
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Status parsing
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct StatusInfo {
    branch: Option<String>,
    upstream: Option<String>,
    ahead: i32,
    behind: i32,
    staged: Vec<StatusEntry>,
    unstaged: Vec<StatusEntry>,
    untracked: Vec<String>,
    merge_conflicts: Vec<String>,
}

#[derive(Serialize, Clone)]
struct StatusEntry {
    path: String,
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    old_path: Option<String>,
}

fn parse_status(output: &str, _path: &Option<String>) -> Result<StatusInfo, String> {
    let mut info = StatusInfo {
        branch: None,
        upstream: None,
        ahead: 0,
        behind: 0,
        staged: Vec::new(),
        unstaged: Vec::new(),
        untracked: Vec::new(),
        merge_conflicts: Vec::new(),
    };

    for line in output.lines() {
        if line.starts_with('#') {
            // Branch/status header lines
            let rest = line[1..].trim();
            if let Some(name) = rest.strip_prefix("branch.head ") {
                info.branch = Some(if name == "(detached)" {
                    None
                } else {
                    Some(name.to_string())
                })
                .flatten();
            } else if let Some(up) = rest.strip_prefix("branch.upstream ") {
                info.upstream = Some(up.to_string());
            } else if let Some(a) = rest.strip_prefix("branch.ab ") {
                // +N -M
                let parts: Vec<&str> = a.split_whitespace().collect();
                if parts.len() == 2 {
                    info.ahead = parts[0].trim_start_matches('+').parse().unwrap_or(0);
                    info.behind = parts[1].trim_start_matches('-').parse().unwrap_or(0);
                }
            }
        } else if line.len() >= 3 {
            let xy = &line[..2];
            let path = &line[3..];
            let entry = StatusEntry {
                path: path.to_string(),
                status: xy.to_string(),
                old_path: None,
            };
            match xy {
                "??" => info.untracked.push(path.to_string()),
                "DD" | "AU" | "UD" | "UA" | "DU" | "AA" | "UU" => {
                    info.merge_conflicts.push(path.to_string());
                }
                _ => {
                    let staged = xy.as_bytes()[0] != b' ';
                    if staged {
                        info.staged.push(entry.clone());
                    }
                    if xy.as_bytes()[1] != b' ' {
                        info.unstaged.push(entry);
                    }
                }
            }
        }
    }

    // If no branch was found from headers, try git branch --show-current
    if info.branch.is_none() {
        info.branch = run_git(&["branch", "--show-current"])
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());
    }

    Ok(info)
}

// ---------------------------------------------------------------------------
// Diff parsing
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct DiffFile {
    file: String,
    status: String,
    hunks: Vec<DiffHunk>,
}

#[derive(Serialize)]
struct DiffHunk {
    old_start: usize,
    old_lines: usize,
    new_start: usize,
    new_lines: usize,
    header: String,
    lines: Vec<DiffLine>,
}

#[derive(Serialize)]
struct DiffLine {
    #[serde(rename = "type")]
    line_type: String,
    content: String,
    old_line: Option<usize>,
    new_line: Option<usize>,
}

fn parse_diff(output: &str) -> Result<Vec<DiffFile>, String> {
    if output.is_empty() {
        return Ok(Vec::new());
    }

    let mut files: Vec<DiffFile> = Vec::new();
    let mut current_file: Option<DiffFile> = None;
    let mut current_hunk: Option<DiffHunk> = None;
    let mut old_ln: usize = 0;
    let mut new_ln: usize = 0;

    for line in output.lines() {
        if line.starts_with("diff --git ") {
            // Finalize previous hunk/file
            if let Some(hunk) = current_hunk.take() {
                if let Some(ref mut file) = current_file {
                    file.hunks.push(hunk);
                }
            }
            if let Some(file) = current_file.take() {
                files.push(file);
            }
            // Extract filename from "diff --git a/xxx b/xxx"
            let parts: Vec<&str> = line.split_whitespace().collect();
            let file = parts
                .get(3)
                .unwrap_or(&"")
                .trim_start_matches("b/")
                .to_string();
            current_file = Some(DiffFile {
                file,
                status: "modified".into(),
                hunks: Vec::new(),
            });
        } else if line.starts_with("--- ") || line.starts_with("+++ ") {
            continue;
        } else if line.starts_with("@@ ") {
            // Finalize previous hunk
            if let Some(hunk) = current_hunk.take() {
                if let Some(ref mut file) = current_file {
                    file.hunks.push(hunk);
                }
            }
            // Parse "@@ -old_start,old_lines +new_start,new_lines @@"
            let rest = line.trim_start_matches("@@ ");
            let parts: Vec<&str> = rest
                .splitn(2, " @@")
                .next()
                .unwrap_or("")
                .split_whitespace()
                .collect();
            let old_range = parts.first().unwrap_or(&"-0,0");
            let new_range = parts.get(1).unwrap_or(&"+0,0");

            let (o_start, o_len) = parse_range(old_range.trim_start_matches('-'));
            let (n_start, n_len) = parse_range(new_range.trim_start_matches('+'));
            old_ln = o_start;
            new_ln = n_start;

            current_hunk = Some(DiffHunk {
                old_start: o_start,
                old_lines: o_len,
                new_start: n_start,
                new_lines: n_len,
                header: line.to_string(),
                lines: Vec::new(),
            });
        } else if let Some(ref mut hunk) = current_hunk {
            let diff_line = if line.starts_with("+") {
                let l = DiffLine {
                    line_type: "add".into(),
                    content: line[1..].to_string(),
                    old_line: None,
                    new_line: Some(new_ln),
                };
                new_ln += 1;
                l
            } else if line.starts_with("-") {
                let l = DiffLine {
                    line_type: "delete".into(),
                    content: line[1..].to_string(),
                    old_line: Some(old_ln),
                    new_line: None,
                };
                old_ln += 1;
                l
            } else {
                let raw_content = if line.starts_with("\\") {
                    line
                } else {
                    &line[..]
                };
                let l = DiffLine {
                    line_type: if line.starts_with("\\") {
                        "warning".into()
                    } else {
                        "context".into()
                    },
                    content: raw_content.to_string(),
                    old_line: Some(old_ln),
                    new_line: Some(new_ln),
                };
                if !line.starts_with("\\") {
                    old_ln += 1;
                    new_ln += 1;
                }
                l
            };
            hunk.lines.push(diff_line);
        }
    }

    // Finalize last hunk/file
    if let Some(hunk) = current_hunk.take() {
        if let Some(ref mut file) = current_file {
            file.hunks.push(hunk);
        }
    }
    if let Some(file) = current_file.take() {
        files.push(file);
    }

    Ok(files)
}

fn parse_range(s: &str) -> (usize, usize) {
    let parts: Vec<&str> = s.split(',').collect();
    let start = parts.first().and_then(|p| p.parse().ok()).unwrap_or(0);
    let len = parts.get(1).and_then(|p| p.parse().ok()).unwrap_or(1);
    (start, len)
}

// ---------------------------------------------------------------------------
// Log parsing
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct CommitInfo {
    hash: String,
    abbreviated_hash: String,
    author: Person,
    committer: Person,
    subject: String,
    body: String,
    refs: String,
}

#[derive(Serialize)]
struct Person {
    name: String,
    email: String,
    date: String,
}

#[derive(Serialize)]
struct BranchInfo {
    current: String,
    branches: Vec<BranchEntry>,
}

#[derive(Serialize)]
struct BranchEntry {
    name: String,
    current: bool,
    upstream: Option<String>,
    ahead: i32,
    behind: i32,
}

fn parse_log(output: &str) -> Result<Vec<CommitInfo>, String> {
    if output.trim().is_empty() {
        return Ok(Vec::new());
    }

    let mut commits = Vec::new();
    for entry in output.split('\0') {
        let entry = entry.trim();
        if entry.is_empty() {
            continue;
        }
        let mut lines = entry.lines();
        let commit = CommitInfo {
            hash: lines.next().unwrap_or("").to_string(),
            abbreviated_hash: lines.next().unwrap_or("").to_string(),
            author: Person {
                name: lines.next().unwrap_or("").to_string(),
                email: lines.next().unwrap_or("").to_string(),
                date: lines.next().unwrap_or("").to_string(),
            },
            committer: Person {
                name: lines.next().unwrap_or("").to_string(),
                email: lines.next().unwrap_or("").to_string(),
                date: lines.next().unwrap_or("").to_string(),
            },
            refs: lines.next().unwrap_or("").to_string(),
            subject: lines.next().unwrap_or("").to_string(),
            body: lines.collect::<Vec<&str>>().join("\n").trim().to_string(),
        };
        commits.push(commit);
    }
    Ok(commits)
}

fn parse_branch_list(output: &str) -> Result<BranchInfo, String> {
    let mut branches = Vec::new();
    let mut current = String::new();

    for line in output.lines() {
        let (is_current, name) = if line.starts_with("* ") {
            (true, line[2..].trim())
        } else if line.starts_with("  ") {
            (false, line[2..].trim())
        } else {
            (false, line.trim())
        };

        if is_current {
            current = name.to_string();
        }

        // Detect remote branches (origin/...)
        let upstream = if name.contains('/') && !name.starts_with("remotes/") {
            Some(format!("origin/{}", name))
        } else {
            None
        };

        branches.push(BranchEntry {
            name: name.to_string(),
            current: is_current,
            upstream,
            ahead: 0,
            behind: 0,
        });
    }

    // Try to get ahead/behind for current branch
    if !current.is_empty() {
        if let Ok(status_out) = run_git(&["status", "--branch", "--porcelain=v1"]) {
            let info = parse_status(&status_out, &None).unwrap_or(StatusInfo {
                branch: Some(current.clone()),
                upstream: None,
                ahead: 0,
                behind: 0,
                staged: Vec::new(),
                unstaged: Vec::new(),
                untracked: Vec::new(),
                merge_conflicts: Vec::new(),
            });
            if let Some(entry) = branches.iter_mut().find(|b| b.current) {
                entry.ahead = info.ahead;
                entry.behind = info.behind;
                entry.upstream = info.upstream;
            }
        }
    }

    Ok(BranchInfo { current, branches })
}
