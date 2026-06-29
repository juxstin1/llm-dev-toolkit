use std::path::{Path, PathBuf};
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

#[cfg(unix)]
fn symlink_file(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn symlink_file(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_file(target, link)
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
fn test_tk_search_no_match() {
    let dir = setup_temp_dir("search");
    let (stdout, stderr, success) = tk(&["search", "test", dir.to_str().unwrap()]);
    assert!(success, "search should exit 0 with no matches");
    assert!(stdout.is_empty(), "no matches expected");
    assert!(
        stderr.is_empty() || stderr.trim().is_empty(),
        "no error output expected"
    );
    cleanup(&dir);
}

#[test]
fn test_tk_grep_alias_finds_match() {
    let dir = setup_temp_dir("grep");
    // The `grep` alias should work and find content inside files.
    let (stdout, _, success) = tk(&["grep", "foo", dir.to_str().unwrap()]);
    assert!(success, "grep alias should succeed");
    assert!(
        stdout.contains("hello.txt"),
        "grep should find 'foo' in hello.txt, got: {stdout}"
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
fn test_tk_clip_help_documents_file_fallback_opt_in() {
    let (stdout, _, success) = tk(&["clip", "--help"]);
    assert!(success, "clip --help should succeed");
    assert!(
        stdout.contains("--allow-file-fallback"),
        "help should document file fallback opt-in, got: {stdout}"
    );
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

#[test]
fn test_tk_tree_all_still_excludes_git_dir() {
    let dir = setup_temp_dir("tree-git");
    std::fs::create_dir(dir.join(".git")).unwrap();
    std::fs::write(dir.join(".git").join("config"), b"[core]\n").unwrap();
    let (stdout, _, success) = tk(&[
        "tree",
        dir.to_str().unwrap(),
        "-a",
        "-L",
        "1",
        "--color",
        "never",
    ]);
    assert!(success, "tree -a should succeed");
    assert!(
        !stdout.contains(".git"),
        "tree -a must not surface .git internals, got: {stdout}"
    );
    cleanup(&dir);
}

#[test]
fn test_tk_ltd_hides_hidden_entries() {
    let dir = setup_temp_dir("ltd-hidden");
    std::fs::write(dir.join(".hidden"), b"secret\n").unwrap();
    std::fs::create_dir(dir.join(".git")).unwrap();
    std::fs::write(dir.join(".git").join("config"), b"[core]\n").unwrap();
    let (stdout, _, success) = tk(&["ltd", "-L", "1", dir.to_str().unwrap(), "--color", "never"]);
    assert!(success, "ltd should succeed");
    assert!(
        stdout.contains("hello.txt"),
        "ltd should show visible files"
    );
    assert!(
        !stdout.contains(".hidden") && !stdout.contains(".git"),
        "ltd must hide dot entries by default, got: {stdout}"
    );
    cleanup(&dir);
}

#[test]
fn test_tk_checksum_invalid_algorithm_fails() {
    let dir = setup_temp_dir("checksum-invalid");
    let file = dir.join("hello.txt");
    let (stdout, stderr, success) = tk(&["checksum", file.to_str().unwrap(), "-a", "sha1"]);
    assert!(!success, "unsupported checksum algorithm should fail");
    assert!(
        stdout.trim().is_empty(),
        "stdout should stay empty on error"
    );
    assert!(
        stderr.contains("Unsupported algorithm"),
        "stderr should explain unsupported algorithm, got: {stderr}"
    );
    cleanup(&dir);
}

#[test]
fn test_tk_ff_rejects_invalid_type_filter() {
    let dir = setup_temp_dir("ff-invalid-type");
    let (_stdout, stderr, success) = tk(&["ff", "hello", dir.to_str().unwrap(), "-t", "x"]);
    assert!(!success, "invalid ff type filter should fail");
    assert!(
        stderr.contains("Invalid type filter"),
        "stderr should explain invalid type filter, got: {stderr}"
    );
    cleanup(&dir);
}

#[test]
fn test_tk_sort_rejects_invalid_sort_field() {
    let dir = setup_temp_dir("sort-invalid-field");
    let (_stdout, stderr, success) = tk(&["sort", dir.to_str().unwrap(), "--by", "nope"]);
    assert!(!success, "invalid sort field should fail");
    assert!(
        stderr.contains("Invalid sort field"),
        "stderr should explain invalid sort field, got: {stderr}"
    );
    cleanup(&dir);
}

#[test]
fn test_tk_dups_rejects_invalid_min_size() {
    let dir = setup_temp_dir("dups-invalid-size");
    let (_stdout, stderr, success) = tk(&["dups", dir.to_str().unwrap(), "-m", "nonsense"]);
    assert!(!success, "invalid duplicate-size filter should fail");
    assert!(
        stderr.contains("Invalid minimum size"),
        "stderr should explain invalid size filter, got: {stderr}"
    );
    cleanup(&dir);
}

#[test]
fn test_tk_spec0_list() {
    let (stdout, _, success) = tk(&["spec0", "list"]);
    assert!(success, "spec0 list should succeed");
    assert!(stdout.contains("/spec0-plan"), "should list plan command");
    assert!(
        stdout.contains("claude"),
        "should list claude install target"
    );
    assert!(stdout.contains("codex"), "should list codex install target");
    assert!(
        stdout.contains("opencode"),
        "should list opencode install target"
    );
}

#[test]
fn test_tk_spec0_print() {
    let (stdout, _, success) = tk(&["spec0", "print", "spec0-plan"]);
    assert!(success, "spec0 print should succeed");
    assert!(
        stdout.contains("# Spec0 Plan"),
        "should print bundled prompt"
    );
    assert!(
        stdout.contains("argument-hint"),
        "should include command front matter"
    );
}

#[test]
fn test_tk_spec0_project_install_claude() {
    let dir = setup_temp_dir("spec0-claude");
    let (stdout, _, success) = tk(&[
        "spec0",
        "install",
        "--agent",
        "claude",
        "--scope",
        "project",
        "--dir",
        dir.to_str().unwrap(),
    ]);
    assert!(success, "spec0 install claude should succeed");
    assert!(
        stdout.contains("spec0-plan.md"),
        "should report written file"
    );
    assert!(
        dir.join(".claude")
            .join("commands")
            .join("spec0-plan.md")
            .exists(),
        "should install Claude command"
    );
    cleanup(&dir);
}

#[test]
fn test_tk_spec0_project_install_codex() {
    let dir = setup_temp_dir("spec0-codex");
    let (stdout, _, success) = tk(&[
        "spec0",
        "install",
        "--agent",
        "codex",
        "--scope",
        "project",
        "--dir",
        dir.to_str().unwrap(),
    ]);
    assert!(success, "spec0 install codex should succeed");
    assert!(stdout.contains("SKILL.md"), "should report written skill");
    assert!(
        dir.join(".agents")
            .join("skills")
            .join("spec0")
            .join("SKILL.md")
            .exists(),
        "should install Codex skill"
    );
    cleanup(&dir);
}

// --- --format json -----------------------------------------------------------

/// Run tk and parse stdout as JSON, asserting success.
fn tk_json(args: &[&str]) -> serde_json::Value {
    let (stdout, stderr, success) = tk(args);
    assert!(success, "command should succeed; stderr: {stderr}");
    serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("output was not valid JSON: {e}\n{stdout}"))
}

fn assert_json_runtime_error(args: &[&str], expected: &str) {
    let (stdout, stderr, success) = tk(args);
    assert!(!success, "command should fail");
    assert!(
        stdout.trim().is_empty(),
        "stdout should stay empty on error, got: {stdout}"
    );
    let value: serde_json::Value = serde_json::from_str(stderr.trim())
        .unwrap_or_else(|e| panic!("stderr was not valid JSON: {e}\n{stderr}"));
    let error = value["error"]
        .as_str()
        .unwrap_or_else(|| panic!("stderr JSON should contain an error string: {value}"));
    assert!(
        error.contains(expected),
        "error should contain {expected:?}, got: {error}"
    );
}

#[test]
fn test_json_ls_is_array_of_typed_entries() {
    let dir = setup_temp_dir("json-ls");
    let v = tk_json(&["ls", dir.to_str().unwrap(), "--format", "json"]);
    let arr = v.as_array().expect("ls json should be an array");
    let names: Vec<&str> = arr.iter().filter_map(|e| e["name"].as_str()).collect();
    assert!(names.contains(&"hello.txt"));
    let subdir = arr.iter().find(|e| e["name"] == "subdir").unwrap();
    assert_eq!(subdir["type"], "dir");
    // Sizes are raw numbers, not formatted strings.
    assert!(arr[0]["size"].is_number(), "size must be numeric in json");
    cleanup(&dir);
}

#[test]
fn test_json_count_carries_all_metrics() {
    let dir = setup_temp_dir("json-count");
    let file = dir.join("hello.txt");
    let v = tk_json(&["count", file.to_str().unwrap(), "--format", "json"]);
    let rec = &v.as_array().unwrap()[0];
    assert_eq!(rec["lines"], 3);
    assert!(rec["words"].is_number());
    assert!(rec["bytes"].is_number());
    cleanup(&dir);
}

#[test]
fn test_json_tree_is_nested() {
    let dir = setup_temp_dir("json-tree");
    let v = tk_json(&["tree", dir.to_str().unwrap(), "--format", "json"]);
    assert_eq!(v["type"], "dir");
    let children = v["children"].as_array().expect("tree should have children");
    let subdir = children.iter().find(|c| c["name"] == "subdir").unwrap();
    assert_eq!(subdir["type"], "dir");
    cleanup(&dir);
}

#[test]
fn test_json_tree_all_still_excludes_git_dir() {
    let dir = setup_temp_dir("json-tree-git");
    std::fs::create_dir(dir.join(".git")).unwrap();
    std::fs::write(dir.join(".git").join("config"), b"[core]\n").unwrap();
    let v = tk_json(&[
        "tree",
        dir.to_str().unwrap(),
        "-a",
        "-L",
        "1",
        "--format",
        "json",
    ]);
    let children = v["children"].as_array().expect("tree should have children");
    assert!(
        !children.iter().any(|child| child["name"] == ".git"),
        "json tree -a must not surface .git"
    );
    cleanup(&dir);
}

#[test]
fn test_json_stats_directory_counts_files_and_dirs_separately() {
    let dir = setup_temp_dir("json-stats-dir");
    let v = tk_json(&["stats", dir.to_str().unwrap(), "-d", "--format", "json"]);
    let rows = v["by_directory"]
        .as_array()
        .expect("stats should include directory rows");
    let root_row = rows
        .iter()
        .find(|row| row["directory"].as_str() == dir.to_str())
        .unwrap_or_else(|| panic!("missing root directory row in {rows:#?}"));
    assert_eq!(root_row["files"], 3);
    assert_eq!(root_row["dirs"], 1);
    cleanup(&dir);
}

#[test]
fn test_json_search_omits_ansi() {
    let dir = setup_temp_dir("json-search");
    let v = tk_json(&["search", "hello", dir.to_str().unwrap(), "--format", "json"]);
    let arr = v.as_array().unwrap();
    assert!(!arr.is_empty(), "should find matches");
    for hit in arr {
        let text = hit["text"].as_str().unwrap();
        assert!(
            !text.contains('\u{1b}'),
            "json text must not contain ANSI escapes"
        );
        assert!(hit["line"].is_number());
    }
    cleanup(&dir);
}

#[test]
fn test_search_extension_filter_accepts_leading_dot() {
    let dir = setup_temp_dir("search-ext-dot");
    let (plain_stdout, plain_stderr, plain_success) =
        tk(&["search", "hello", dir.to_str().unwrap(), "-e", "rs"]);
    let (dotted_stdout, dotted_stderr, dotted_success) =
        tk(&["search", "hello", dir.to_str().unwrap(), "-e", ".rs"]);
    assert!(
        plain_success,
        "plain extension search failed: {plain_stderr}"
    );
    assert!(
        dotted_success,
        "dotted extension search failed: {dotted_stderr}"
    );
    assert_eq!(plain_stdout, dotted_stdout);
    assert!(
        dotted_stdout.contains("test.rs"),
        "dotted extension search should find test.rs, got: {dotted_stdout}"
    );
    cleanup(&dir);
}

#[test]
fn test_json_search_extension_filter_accepts_leading_dot() {
    let dir = setup_temp_dir("json-search-ext-dot");
    let plain = tk_json(&[
        "search",
        "hello",
        dir.to_str().unwrap(),
        "-e",
        "rs",
        "--format",
        "json",
    ]);
    let dotted = tk_json(&[
        "search",
        "hello",
        dir.to_str().unwrap(),
        "-e",
        ".rs",
        "--format",
        "json",
    ]);
    assert_eq!(plain, dotted);
    assert!(
        !dotted.as_array().unwrap().is_empty(),
        "dotted extension JSON search should find matches"
    );
    cleanup(&dir);
}

#[test]
fn test_json_info_reports_symlink_path_type() {
    let dir = setup_temp_dir("info-symlink");
    let target = dir.join("hello.txt");
    let link = dir.join("hello-link.txt");
    if symlink_file(&target, &link).is_err() {
        cleanup(&dir);
        return;
    }
    let v = tk_json(&["info", "-f", link.to_str().unwrap(), "--format", "json"]);
    assert_eq!(v["type"], "symlink");
    cleanup(&dir);
}

#[test]
fn test_json_runtime_error_for_invalid_checksum_algorithm() {
    let dir = setup_temp_dir("json-error-checksum");
    let file = dir.join("hello.txt");
    assert_json_runtime_error(
        &[
            "checksum",
            file.to_str().unwrap(),
            "-a",
            "sha1",
            "--format",
            "json",
        ],
        "Unsupported algorithm",
    );
    cleanup(&dir);
}

#[test]
fn test_json_runtime_error_for_non_object_json_keys() {
    let dir = setup_temp_dir("json-error-keys");
    let input_path = dir.join("array.json");
    std::fs::write(&input_path, b"[1,2,3]").unwrap();
    assert_json_runtime_error(
        &[
            "json",
            "keys",
            input_path.to_str().unwrap(),
            "--format",
            "json",
        ],
        "JSON value is not an object",
    );
    cleanup(&dir);
}

// --- mcp server --------------------------------------------------------------

/// Feed newline-delimited JSON-RPC to `tk mcp` over stdin, return decoded
/// responses (one per non-empty output line).
fn tk_mcp(requests: &[&str]) -> Vec<serde_json::Value> {
    use std::io::Write;
    use std::process::Stdio;

    let mut child = Command::new(tk_binary())
        .arg("mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn tk mcp");
    let input = requests.join("\n");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("each mcp response is a JSON line"))
        .collect()
}

#[test]
fn test_mcp_initialize_and_list_tools() {
    let resps = tk_mcp(&[
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18"}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#,
    ]);
    // The notification produces no response, so two responses for three inputs.
    assert_eq!(resps.len(), 2, "notification must not get a reply");
    assert_eq!(resps[0]["result"]["serverInfo"]["name"], "tk");
    let names: Vec<&str> = resps[1]["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|t| t["name"].as_str())
        .collect();
    assert!(names.contains(&"search"));
    assert!(
        !names.contains(&"extract"),
        "side-effecting tools are not exposed"
    );
}

#[test]
fn test_mcp_tools_call_returns_json_content() {
    let dir = setup_temp_dir("mcp-call");
    let req = format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{{"name":"count","arguments":{{"files":["{}"]}}}}}}"#,
        dir.join("hello.txt")
            .to_str()
            .unwrap()
            .replace('\\', "\\\\")
    );
    let resps = tk_mcp(&[&req]);
    let result = &resps[0]["result"];
    assert_eq!(result["isError"], false);
    let text = result["content"][0]["text"].as_str().unwrap();
    // The embedded text is itself the command's JSON output.
    let inner: serde_json::Value = serde_json::from_str(text).unwrap();
    assert_eq!(inner[0]["lines"], 3);
    cleanup(&dir);
}

#[test]
fn test_mcp_checksum_invalid_algorithm_reports_tool_error() {
    let req = r#"{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"checksum","arguments":{"files":["Cargo.toml"],"algorithm":"sha1"}}}"#;
    let resps = tk_mcp(&[req]);
    let result = &resps[0]["result"];
    assert_eq!(result["isError"], true);
    let text = result["content"][0]["text"].as_str().unwrap();
    assert!(
        text.contains("Unsupported algorithm"),
        "tool error should explain unsupported algorithm, got: {text}"
    );
}

#[test]
fn test_tk_read_file_basic() {
    let dir = setup_temp_dir("read-file-basic");
    let (stdout, stderr, success) = tk(&["read-file", dir.join("hello.txt").to_str().unwrap()]);
    assert!(success, "read-file should succeed");
    assert!(stderr.is_empty(), "no stderr on success");
    assert!(
        stdout.contains("hello world"),
        "should contain file content"
    );
    assert!(stdout.contains("     1:"), "should show line numbers");
    cleanup(&dir);
}

#[test]
fn test_tk_read_file_with_offset_and_limit() {
    let dir = setup_temp_dir("read-file-offset-limit");
    let (stdout, _, success) = tk(&[
        "read-file",
        dir.join("hello.txt").to_str().unwrap(),
        "--offset",
        "2",
        "--limit",
        "2",
    ]);
    assert!(success, "read-file with offset/limit should succeed");
    assert!(stdout.contains("     2:"), "should start at line 2");
    assert!(
        !stdout.contains("     4:"),
        "should not include beyond limit"
    );
    cleanup(&dir);
}

#[test]
fn test_tk_read_file_binary_rejected() {
    let dir = setup_temp_dir("read-file-binary");
    let (_, stderr, success) = tk(&["read-file", dir.join("logo.bin").to_str().unwrap()]);
    assert!(!success, "read-file should reject binary files");
    assert!(stderr.contains("binary"), "should mention binary");
    cleanup(&dir);
}

#[test]
fn test_tk_read_file_nonexistent() {
    let (_, stderr, success) = tk(&["read-file", "/nonexistent/path.txt"]);
    assert!(!success, "read-file should fail on nonexistent file");
    assert!(stderr.contains("not found"), "should mention not found");
}

#[test]
fn test_tk_read_lines_basic() {
    let dir = setup_temp_dir("read-lines-basic");
    let (stdout, _stderr, success) = tk(&[
        "read-lines",
        dir.join("hello.txt").to_str().unwrap(),
        "--start-line",
        "1",
        "--end-line",
        "2",
    ]);
    assert!(success, "read-lines should succeed");
    assert!(stdout.contains("hello world"), "should include first line");
    assert!(stdout.contains("foo bar"), "should include second line");
    assert!(!stdout.contains("baz"), "should not include third line");
    cleanup(&dir);
}

#[test]
fn test_tk_read_file_json_output() {
    let dir = setup_temp_dir("read-file-json");
    let (stdout, _, success) = tk(&[
        "read-file",
        dir.join("hello.txt").to_str().unwrap(),
        "--format",
        "json",
    ]);
    assert!(success, "read-file --format json should succeed");
    let v: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(
        v["path"].as_str().unwrap(),
        dir.join("hello.txt").to_str().unwrap()
    );
    assert_eq!(v["total_lines"], 3);
    assert_eq!(v["returned_lines"], 3);
    assert_eq!(v["truncated"], false);
    cleanup(&dir);
}

fn escape_path(path: &std::path::Path) -> String {
    path.to_str().unwrap().replace('\\', "\\\\")
}

#[test]
fn test_mcp_read_file_returns_content() {
    let dir = setup_temp_dir("mcp-read-file");
    let path = escape_path(&dir.join("hello.txt"));
    let req = format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{{"name":"read_file","arguments":{{"path":"{}"}}}}}}"#,
        path
    );
    let resps = tk_mcp(&[&req]);
    let result = &resps[0]["result"];
    assert!(
        !result["isError"].as_bool().unwrap(),
        "read_file should not error"
    );
    let text = result["content"][0]["text"].as_str().unwrap();
    assert!(
        text.contains("hello world"),
        "MCP read_file should contain file content"
    );
    cleanup(&dir);
}

#[test]
fn test_mcp_read_lines_returns_range() {
    let dir = setup_temp_dir("mcp-read-lines");
    let path = escape_path(&dir.join("hello.txt"));
    let req = format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{{"name":"read_lines","arguments":{{"path":"{}","start_line":1,"end_line":2}}}}}}"#,
        path
    );
    let resps = tk_mcp(&[&req]);
    let result = &resps[0]["result"];
    assert!(
        !result["isError"].as_bool().unwrap(),
        "read_lines should not error"
    );
    let text = result["content"][0]["text"].as_str().unwrap();
    assert!(
        text.contains("hello world"),
        "MCP read_lines should contain first line"
    );
    assert!(
        text.contains("foo bar"),
        "MCP read_lines should contain second line"
    );
    cleanup(&dir);
}

#[test]
fn test_mcp_fetch_tool_listed() {
    let req = r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#;
    let resps = tk_mcp(&[req]);
    let tools = &resps[0]["result"]["tools"];
    let names: Vec<&str> = tools
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|t| t["name"].as_str())
        .collect();
    assert!(names.contains(&"fetch"), "fetch tool should be listed");
    assert!(names.contains(&"scrape"), "scrape tool should be listed");
}

#[test]
fn test_tk_context_with_include_glob() {
    let dir = setup_temp_dir("context-include");
    let (stdout, _, success) = tk(&["context", dir.to_str().unwrap(), "--include", "*.rs"]);
    assert!(success, "context --include should succeed");
    assert!(stdout.contains("test.rs"), "should include .rs file");
    assert!(!stdout.contains("hello.txt"), "should exclude .txt file");
    cleanup(&dir);
}

#[test]
fn test_tk_context_with_exclude_glob() {
    let dir = setup_temp_dir("context-exclude");
    let (stdout, _, success) = tk(&["context", dir.to_str().unwrap(), "--exclude", "*.bin"]);
    assert!(success, "context --exclude should succeed");
    assert!(stdout.contains("hello.txt"), "should include .txt file");
    assert!(stdout.contains("test.rs"), "should include .rs file");
    assert!(!stdout.contains("logo.bin"), "should exclude .bin file");
    cleanup(&dir);
}
