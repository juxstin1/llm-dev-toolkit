use clap::Args;
use serde::Serialize;
use std::path::Path;

#[derive(Args)]
pub struct ScanArgs {
    path: Option<String>,
    #[arg(short = 'L', long, default_value = "2", help = "Tree depth")]
    depth: usize,
    #[arg(
        short = 'n',
        long,
        default_value = "10",
        help = "Largest/recent item count"
    )]
    count: usize,
}

#[derive(Serialize)]
struct ScanResult {
    path: String,
    tree: TreeNode,
    stats: ScanStats,
    largest: Vec<FileEntry>,
    recent: Vec<RecentEntry>,
}

#[derive(Serialize)]
struct TreeNode {
    name: String,
    #[serde(rename = "type")]
    kind: String,
    children: Vec<TreeNode>,
}

#[derive(Serialize)]
struct ScanStats {
    files: usize,
    dirs: usize,
    bytes: u64,
}

#[derive(Serialize)]
struct FileEntry {
    path: String,
    size: u64,
}

#[derive(Serialize)]
struct RecentEntry {
    path: String,
    modified: Option<i64>,
    size: u64,
}

fn insert_tree(nodes: &mut Vec<TreeNode>, parts: &[String], leaf_is_dir: bool) {
    let Some(name) = parts.first() else {
        return;
    };
    let is_leaf = parts.len() == 1;
    let kind = if is_leaf && !leaf_is_dir {
        "file"
    } else {
        "dir"
    };
    let index = nodes
        .iter()
        .position(|node| node.name == *name)
        .unwrap_or_else(|| {
            nodes.push(TreeNode {
                name: name.clone(),
                kind: kind.into(),
                children: Vec::new(),
            });
            nodes.len() - 1
        });
    if !is_leaf {
        insert_tree(&mut nodes[index].children, &parts[1..], leaf_is_dir);
    }
}

fn sort_tree(node: &mut TreeNode) {
    node.children
        .sort_by(|a, b| (a.kind.as_str(), &a.name).cmp(&(b.kind.as_str(), &b.name)));
    for child in &mut node.children {
        sort_tree(child);
    }
}

pub fn run(args: &ScanArgs) -> Result<(), String> {
    let root = args.path.as_deref().unwrap_or(".");
    let root_path = Path::new(root);
    if !root_path.is_dir() {
        return Err(format!("scan root '{}' is not a directory", root));
    }
    let root_name = root_path
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| root.to_string());
    let mut tree = TreeNode {
        name: root_name,
        kind: "dir".into(),
        children: Vec::new(),
    };
    let mut stats = ScanStats {
        files: 0,
        dirs: 0,
        bytes: 0,
    };
    let mut largest = Vec::new();
    let mut recent = Vec::new();
    let walk = super::WalkConfig {
        root,
        show_all: false,
        max_depth: None,
    };

    for entry in super::walk_entries(&walk)? {
        if entry.depth() == 0 {
            continue;
        }
        let path = entry.path();
        let relative = path.strip_prefix(root_path).unwrap_or(path);
        let parts = relative
            .components()
            .map(|part| part.as_os_str().to_string_lossy().to_string())
            .collect::<Vec<_>>();
        let file_type = match entry.file_type() {
            Some(file_type) => file_type,
            None => continue,
        };
        if entry.depth() <= args.depth {
            insert_tree(&mut tree.children, &parts, file_type.is_dir());
        }
        if file_type.is_dir() {
            stats.dirs += 1;
            continue;
        }
        if !file_type.is_file() {
            continue;
        }
        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(_) => continue,
        };
        stats.files += 1;
        stats.bytes = stats.bytes.saturating_add(metadata.len());
        let display = relative.to_string_lossy().to_string();
        largest.push(FileEntry {
            path: display.clone(),
            size: metadata.len(),
        });
        let modified = metadata.modified().ok().and_then(|time| {
            time.duration_since(std::time::UNIX_EPOCH)
                .ok()
                .map(|duration| duration.as_secs() as i64)
        });
        recent.push(RecentEntry {
            path: display,
            modified,
            size: metadata.len(),
        });
    }

    sort_tree(&mut tree);
    largest.sort_by_key(|entry| std::cmp::Reverse(entry.size));
    largest.truncate(args.count);
    recent.sort_by_key(|entry| std::cmp::Reverse(entry.modified.unwrap_or(0)));
    recent.truncate(args.count);
    let result = ScanResult {
        path: root.to_string(),
        tree,
        stats,
        largest,
        recent,
    };

    if super::json_enabled() {
        super::emit_json(&result)
    } else {
        println!("Path: {}", result.path);
        println!(
            "Stats: {} files, {} directories, {}",
            result.stats.files,
            result.stats.dirs,
            super::format_size(result.stats.bytes)
        );
        println!("Tree:");
        print_tree(&result.tree, 0);
        println!("Largest:");
        for entry in &result.largest {
            println!("  {:>10}  {}", super::format_size(entry.size), entry.path);
        }
        println!("Recent:");
        for entry in &result.recent {
            println!("  {}", entry.path);
        }
        Ok(())
    }
}

fn print_tree(node: &TreeNode, depth: usize) {
    println!("{}{}", "  ".repeat(depth), node.name);
    for child in &node.children {
        print_tree(child, depth + 1);
    }
}
