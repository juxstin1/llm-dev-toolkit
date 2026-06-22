use std::path::Path;

use serde::Serialize;

use super::{ansi, paint};
use crate::{LtdArgsInner, TreeArgs};

#[derive(Serialize)]
struct Node {
    name: String,
    #[serde(rename = "type")]
    kind: &'static str,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    children: Vec<Node>,
}

pub fn run(args: &TreeArgs) -> Result<(), String> {
    let root = args.path.as_deref().unwrap_or(".");
    let path = Path::new(root);
    if !path.is_dir() {
        return Err(format!("Not a directory: {}", root));
    }
    if super::json_enabled() {
        let node = build_node(
            path,
            root.to_string(),
            true,
            args.depth,
            args.all,
            args.dirs_only,
        );
        return super::emit_json(&node);
    }
    println!("{}", root);
    print_tree(root, args.depth, args.all, args.dirs_only, "")
}

pub fn run_depth(args: &LtdArgsInner) -> Result<(), String> {
    let root = args.path.as_deref().unwrap_or(".");
    let path = Path::new(root);
    if !path.is_dir() {
        return Err(format!("Not a directory: {}", root));
    }
    if super::json_enabled() {
        let node = build_node(path, root.to_string(), true, args.depth, false, false);
        return super::emit_json(&node);
    }
    println!("{}", root);
    print_tree(root, args.depth, false, false, "")
}

fn is_git_path(path: &Path) -> bool {
    path.components().any(|c| c.as_os_str() == ".git")
}

/// Build a nested tree of [`Node`]s. `levels` mirrors the text renderer's depth
/// semantics: `Some(n)` shows `n` levels of entries below `path`, `None` is
/// unbounded.
fn build_node(
    path: &Path,
    name: String,
    is_dir: bool,
    levels: Option<usize>,
    show_all: bool,
    dirs_only: bool,
) -> Node {
    let kind = if is_dir { "dir" } else { "file" };
    let mut children = Vec::new();

    let descend = is_dir && levels.is_none_or(|l| l > 0);
    if descend {
        let mut entries: Vec<_> = ignore::WalkBuilder::new(path)
            .max_depth(Some(1))
            .hidden(!show_all)
            .parents(false)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .build()
            .filter_map(|e| e.ok())
            .filter(|e| e.path() != path)
            .filter(|e| !is_git_path(e.path()))
            .filter(|e| show_all || !e.file_name().to_string_lossy().starts_with('.'))
            .filter(|e| !dirs_only || e.file_type().is_some_and(|ft| ft.is_dir()))
            .collect();
        entries.sort_by(|a, b| a.file_name().cmp(b.file_name()));

        for entry in entries {
            let child_is_dir = entry.file_type().is_some_and(|ft| ft.is_dir());
            let child_name = entry.file_name().to_string_lossy().to_string();
            children.push(build_node(
                entry.path(),
                child_name,
                child_is_dir,
                levels.map(|l| l - 1),
                show_all,
                dirs_only,
            ));
        }
    }

    Node {
        name,
        kind,
        children,
    }
}

fn print_tree(
    root: &str,
    max_depth: Option<usize>,
    show_all: bool,
    dirs_only: bool,
    prefix: &str,
) -> Result<(), String> {
    let mut entries: Vec<_> = ignore::WalkBuilder::new(root)
        .max_depth(Some(1))
        .hidden(!show_all)
        .parents(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build()
        .filter_map(|e| e.ok())
        .filter(|e| e.path() != Path::new(root))
        .filter(|e| !is_git_path(e.path()))
        .filter(|e| show_all || !e.file_name().to_string_lossy().starts_with('.'))
        .filter(|e| !dirs_only || e.file_type().is_some_and(|ft| ft.is_dir()))
        .collect();

    entries.sort_by(|a, b| a.file_name().cmp(b.file_name()));

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let connector = if is_last { "└── " } else { "├── " };

        let name = entry.file_name().to_string_lossy();
        let label = if entry.file_type().is_some_and(|ft| ft.is_dir()) {
            paint(ansi::BLUE, &name)
        } else {
            name.to_string()
        };
        println!("{}{}{}", prefix, connector, label);

        if entry.file_type().is_some_and(|ft| ft.is_dir()) {
            let child_prefix = if is_last { "    " } else { "│   " };
            let new_prefix = format!("{}{}", prefix, child_prefix);

            let sub_path = entry.path().to_string_lossy().to_string();
            let current_depth = prefix.chars().filter(|c| *c == '│' || *c == ' ').count() / 4;

            let should_recurse = match max_depth {
                Some(d) => current_depth + 1 < d,
                None => true,
            };

            if should_recurse {
                let _ = print_tree(&sub_path, max_depth, show_all, dirs_only, &new_prefix);
            }
        }
    }

    Ok(())
}
