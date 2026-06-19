use std::path::PathBuf;
use std::process::Command;

fn tk_binary() -> PathBuf {
    let mut p = std::env::current_exe().unwrap();
    p.pop();
    if p.ends_with("deps") {
        p.pop();
    }
    p.push(if cfg!(windows) { "tk.exe" } else { "tk" });
    p
}

fn tk(args: &[&str]) -> (String, String, bool) {
    let output = Command::new(tk_binary())
        .args(args)
        .output()
        .expect("failed to execute tk");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (stdout, stderr, output.status.success())
}

fn setup_temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("tk-integration-{}", name));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("hello.txt"), b"hello world\nfoo bar\nbaz\n").unwrap();
    std::fs::write(
        dir.join("test.rs"),
        b"fn main() {\n    println!(\"hello\");\n}\n",
    )
    .unwrap();
    std::fs::create_dir(dir.join("subdir")).unwrap();
    std::fs::write(dir.join("subdir").join("nested.txt"), b"nested content\n").unwrap();
    // A binary blob (contains a NUL byte) to exercise binary handling.
    std::fs::write(dir.join("logo.bin"), b"\x89PNG\x00\x00binary").unwrap();
    dir
}

fn cleanup(dir: &PathBuf) {
    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn test_tk_ls() {
    let dir = setup_temp_dir("ls");
    let (stdout, _, success) = tk(&["ls", dir.to_str().unwrap()]);
    assert!(success, "ls should succeed");
    assert!(stdout.contains("hello.txt"), "should list hello.txt");
    assert!(stdout.contains("test.rs"), "should list test.rs");
    assert!(stdout.contains("subdir"), "should list subdir");
    cleanup(&dir);
}

#[test]
fn test_tk_tree() {
    let dir = setup_temp_dir("tree");
    let (stdout, _, success) = tk(&["tree", dir.to_str().unwrap()]);
    assert!(success, "tree should succeed");
    assert!(!stdout.is_empty(), "tree output should not be empty");
    cleanup(&dir);
}

#[test]
fn test_tk_ff() {
    let dir = setup_temp_dir("ff");
    let (stdout, _, success) = tk(&["ff", "test", dir.to_str().unwrap()]);
    assert!(success, "ff should succeed");
    assert!(stdout.contains("test.rs"), "should find test.rs");
    cleanup(&dir);
}

#[test]
fn test_tk_rg() {
    let dir = setup_temp_dir("rg");
    let (stdout, stderr, success) = tk(&["rg", "test", dir.to_str().unwrap()]);
    assert!(success, "rg should succeed exit 0 with no matches");
    assert!(stdout.is_empty(), "no matches expected");
    assert!(
        stderr.is_empty() || stderr.trim().is_empty(),
        "no error output expected"
    );
    cleanup(&dir);
}

#[test]
fn test_tk_stats() {
    let dir = setup_temp_dir("stats");
    let (stdout, _, success) = tk(&["stats", dir.to_str().unwrap()]);
    assert!(success, "stats should succeed");
    assert!(stdout.contains("Files"), "should show Files count");
    cleanup(&dir);
}

#[test]
fn test_tk_count() {
    let (stdout, _, success) = tk(&["count", "-l", "-w", "-c", "tests/cli.rs"]);
    assert!(success, "count should succeed");
    assert!(stdout.contains("cli.rs"), "output should contain filename");
}

#[test]
fn test_tk_json_format() {
    let dir = setup_temp_dir("json-fmt");
    let input_path = dir.join("input.json");
    std::fs::write(&input_path, b"{\"a\":1}").unwrap();
    let (stdout, _, success) = tk(&["json", "format", input_path.to_str().unwrap()]);
    assert!(success, "json format should succeed");
    assert!(
        stdout.contains("\"a\""),
        "formatted output should contain key"
    );
    cleanup(&dir);
}

#[test]
fn test_tk_json_validate_invalid() {
    let dir = setup_temp_dir("json-val");
    let input_path = dir.join("bad.json");
    std::fs::write(&input_path, b"not json").unwrap();
    let (stdout, _, success) = tk(&["json", "validate", input_path.to_str().unwrap()]);
    assert!(success, "json validate should succeed");
    assert!(stdout.contains("invalid"), "should report invalid JSON");
    cleanup(&dir);
}

#[test]
fn test_tk_info() {
    let (stdout, _, success) = tk(&["info", "-f", "tests/cli.rs"]);
    assert!(success, "info should succeed");
    assert!(stdout.contains("cli.rs"), "should show filename");
}

#[test]
fn test_tk_sort() {
    let dir = setup_temp_dir("sort");
    let (stdout, _, success) = tk(&["sort", dir.to_str().unwrap()]);
    assert!(success, "sort should succeed");
    assert!(!stdout.is_empty(), "sort output should not be empty");
    cleanup(&dir);
}

#[test]
fn test_tk_color_never_suppresses_ansi() {
    let dir = setup_temp_dir("color");
    let (stdout, _, success) = tk(&["--color=never", "ls", dir.to_str().unwrap()]);
    assert!(success, "ls with --color=never should succeed");
    assert!(
        !stdout.contains('\x1b'),
        "output should not contain ANSI escape codes"
    );
    cleanup(&dir);
}

#[test]
fn test_tk_color_always_forces_ansi() {
    let dir = setup_temp_dir("color-always");
    let (stdout, _, success) = tk(&["--color=always", "ls", dir.to_str().unwrap()]);
    assert!(success, "ls with --color=always should succeed");
    assert!(
        stdout.contains('\x1b'),
        "output should contain ANSI escape codes"
    );
    cleanup(&dir);
}

#[test]
fn test_tk_ff_name_substring() {
    let dir = setup_temp_dir("ffname");
    let (stdout, _, success) = tk(&["ff-name", "hello", dir.to_str().unwrap()]);
    assert!(success, "ff-name should succeed");
    assert!(stdout.contains("hello.txt"), "should find hello.txt");
    cleanup(&dir);
}

#[test]
fn test_tk_ff_name_glob() {
    let dir = setup_temp_dir("ffglob");
    let (stdout, _, success) = tk(&["ff-name", "--glob", "*.rs", dir.to_str().unwrap()]);
    assert!(success, "ff-name --glob should succeed");
    assert!(stdout.contains("test.rs"), "should find test.rs via glob");
    assert!(
        !stdout.contains("hello.txt"),
        "should not match hello.txt via *.rs glob"
    );
    cleanup(&dir);
}

#[test]
fn test_tk_ff_finds_binary_files() {
    let dir = setup_temp_dir("ff-bin");
    let (stdout, _, success) = tk(&["ff", "logo", dir.to_str().unwrap()]);
    assert!(success, "ff should succeed");
    assert!(
        stdout.contains("logo.bin"),
        "ff should surface binary files by name, got: {stdout}"
    );
    cleanup(&dir);
}

#[test]
fn test_tk_ff_ext_finds_binary_files() {
    let dir = setup_temp_dir("ff-ext-bin");
    let (stdout, _, success) = tk(&["ff-ext", "bin", dir.to_str().unwrap()]);
    assert!(success, "ff-ext should succeed");
    assert!(
        stdout.contains("logo.bin"),
        "ff-ext should surface binary files by extension, got: {stdout}"
    );
    cleanup(&dir);
}

#[test]
fn test_tk_recent_color_never() {
    let dir = setup_temp_dir("recent-color");
    let (stdout, _, success) = tk(&["--color=never", "recent", dir.to_str().unwrap()]);
    assert!(success, "recent should succeed");
    assert!(
        !stdout.contains('\x1b'),
        "recent --color=never should emit no ANSI"
    );
    cleanup(&dir);
}

#[test]
fn test_tk_largest_color_never() {
    let dir = setup_temp_dir("largest-color");
    let (stdout, _, success) = tk(&["--color=never", "largest", "-d", dir.to_str().unwrap()]);
    assert!(success, "largest should succeed");
    assert!(
        !stdout.contains('\x1b'),
        "largest --color=never should emit no ANSI"
    );
    cleanup(&dir);
}

#[test]
fn test_tk_sort_color_never() {
    let dir = setup_temp_dir("sort-color");
    let (stdout, _, success) = tk(&["--color=never", "sort", dir.to_str().unwrap()]);
    assert!(success, "sort should succeed");
    assert!(
        !stdout.contains('\x1b'),
        "sort --color=never should emit no ANSI"
    );
    cleanup(&dir);
}

#[test]
fn test_tk_largest_excludes_git_dir() {
    let dir = setup_temp_dir("largest-git");
    // A file living inside a .git directory must never appear in walk output,
    // even though `largest` walks with show_all = true.
    std::fs::create_dir(dir.join(".git")).unwrap();
    std::fs::write(dir.join(".git").join("config"), vec![b'x'; 4096]).unwrap();
    let (stdout, _, success) = tk(&["largest", dir.to_str().unwrap()]);
    assert!(success, "largest should succeed");
    assert!(
        !stdout.contains(".git"),
        "largest must not descend into .git, got: {stdout}"
    );
    cleanup(&dir);
}

#[test]
fn test_tk_tree_ignore_crate() {
    let dir = setup_temp_dir("tree-ignore");
    let (stdout, _, success) = tk(&["tree", dir.to_str().unwrap()]);
    assert!(success, "tree should succeed");
    assert!(stdout.contains("hello.txt"), "tree should show files");
    assert!(stdout.contains("subdir"), "tree should show subdir");
    cleanup(&dir);
}
