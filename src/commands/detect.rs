#![allow(clippy::type_complexity)]

use clap::Args;
use std::collections::HashMap;
use std::path::Path;

#[derive(Args)]
pub struct DetectArgs {
    path: Option<String>,
    #[arg(long, short = 'v', help = "Show detailed detection info")]
    verbose: bool,
}

#[derive(serde::Serialize)]
pub struct ProjectInfo {
    name: Option<String>,
    version: Option<String>,
    language: String,
    build_system: Option<String>,
    test_framework: Option<String>,
    linter: Option<String>,
    formatter: Option<String>,
    config_files: HashMap<String, bool>,
    scripts: HashMap<String, String>,
}

pub fn run(args: &DetectArgs) -> Result<(), String> {
    crate::config::require_feature("detect")?;

    let root = args.path.as_deref().unwrap_or(".");
    let info = detect(root);

    if crate::commands::json_enabled() {
        crate::commands::emit_json(&info)
    } else {
        print_text(&info, args.verbose);
        Ok(())
    }
}

fn detect(root: &str) -> ProjectInfo {
    let config_files = check_config_files(root);

    let mut info = ProjectInfo {
        name: None,
        version: None,
        language: "Unknown".into(),
        build_system: None,
        test_framework: None,
        linter: None,
        formatter: None,
        config_files,
        scripts: HashMap::new(),
    };

    // Check project types in priority order
    if let Some(rust) = detect_rust(root) {
        info.language = "Rust".into();
        info.build_system = Some("cargo".into());
        info.test_framework = Some("cargo test".into());
        info.linter = Some("cargo clippy".into());
        info.formatter = Some("rustfmt".into());
        info.name = rust.0;
        info.version = rust.1;
        info.scripts
            .insert("test".into(), "cargo test --all".into());
        info.scripts
            .insert("build".into(), "cargo build --release".into());
        info.scripts.insert(
            "lint".into(),
            "cargo clippy --all-targets -- -D warnings".into(),
        );
        info.scripts
            .insert("format".into(), "cargo fmt --all -- --check".into());
        return info;
    }

    if let Some(ts) = detect_typescript(root) {
        info.language = "TypeScript".into();
        info.build_system = Some(ts.0);
        info.test_framework = ts.1;
        info.linter = Some("eslint".into());
        info.formatter = Some("prettier".into());
        info.name = ts.2;
        info.version = ts.3;
        info.scripts.insert("test".into(), ts.4.clone());
        info.scripts.insert("build".into(), ts.5.clone());
        info.scripts.insert("lint".into(), "npx eslint .".into());
        return info;
    }

    if let Some(js) = detect_javascript(root) {
        info.language = "JavaScript".into();
        info.build_system = Some(js.0);
        info.test_framework = js.1;
        info.linter = Some("eslint".into());
        info.formatter = Some("prettier".into());
        info.name = js.2;
        info.version = js.3;
        info.scripts.insert("test".into(), js.4.clone());
        info.scripts.insert("build".into(), js.5.clone());
        info.scripts.insert("lint".into(), "npx eslint .".into());
        return info;
    }

    if let Some(py) = detect_python(root) {
        info.language = "Python".into();
        info.build_system = Some(py.0);
        info.test_framework = py.1;
        info.linter = Some("ruff".into());
        info.formatter = Some("ruff-format".into());
        info.name = py.2;
        info.version = py.3;
        info.scripts.insert("test".into(), py.4.clone());
        return info;
    }

    if let Some(go) = detect_go(root) {
        info.language = "Go".into();
        info.build_system = Some("go build".into());
        info.test_framework = Some("go test".into());
        info.linter = Some("golangci-lint".into());
        info.formatter = Some("gofmt".into());
        info.name = go.0;
        info.version = go.1;
        info.scripts.insert("test".into(), "go test ./...".into());
        info.scripts.insert("build".into(), "go build ./...".into());
        return info;
    }

    if let Some(java) = detect_java(root) {
        info.language = "Java".into();
        info.build_system = Some(java.0);
        info.test_framework = Some(java.1);
        info.linter = Some("checkstyle".into());
        info.name = java.2;
        info.version = java.3;
        info.scripts.insert("test".into(), java.4.clone());
        info.scripts.insert("build".into(), java.5.clone());
        return info;
    }

    // Fallback: detect language by source extension counts
    if let Some(lang) = detect_by_extension(root) {
        info.language = lang;
    }

    info
}

fn check_config_files(root: &str) -> HashMap<String, bool> {
    let candidates = [
        "Cargo.toml",
        "package.json",
        "tsconfig.json",
        "pyproject.toml",
        "setup.py",
        "requirements.txt",
        "go.mod",
        "pom.xml",
        "build.gradle",
        "Dockerfile",
        "Makefile",
        ".github/workflows/ci.yml",
        "rust-toolchain.toml",
        ".rustfmt.toml",
        ".eslintrc.js",
        ".prettierrc",
        ".gitignore",
        "README.md",
    ];
    let mut files = HashMap::new();
    for name in &candidates {
        files.insert(name.to_string(), Path::new(root).join(name).exists());
    }
    files
}

fn read_toml_value(root: &str, file: &str, key: &str) -> Option<String> {
    let path = Path::new(root).join(file);
    let content = std::fs::read_to_string(path).ok()?;
    let value: toml::Value = content.parse().ok()?;
    value
        .get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn read_json_value(root: &str, file: &str, key: &str) -> Option<String> {
    let path = Path::new(root).join(file);
    let content = std::fs::read_to_string(path).ok()?;
    let value: serde_json::Value = content.parse().ok()?;
    value
        .get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn detect_rust(root: &str) -> Option<(Option<String>, Option<String>)> {
    if !Path::new(root).join("Cargo.toml").exists() {
        return None;
    }
    let name = read_toml_value(root, "Cargo.toml", "package.name");
    let version = read_toml_value(root, "Cargo.toml", "package.version");
    Some((name, version))
}

fn detect_typescript(
    root: &str,
) -> Option<(
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    String,
    String,
)> {
    let pkg = Path::new(root).join("package.json");
    let ts = Path::new(root).join("tsconfig.json");
    if !pkg.exists() || !ts.exists() {
        return None;
    }
    let name = read_json_value(root, "package.json", "name");
    let version = read_json_value(root, "package.json", "version");
    let (pm, test_cmd, build_cmd) = detect_npm_tooling(root);
    let test = test_cmd.clone().unwrap_or_else(|| "npm test".into());
    let build = build_cmd.clone().unwrap_or_else(|| "npm run build".into());
    Some((pm, test_cmd, name, version, test, build))
}

fn detect_javascript(
    root: &str,
) -> Option<(
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    String,
    String,
)> {
    let pkg = Path::new(root).join("package.json");
    if !pkg.exists() {
        return None;
    }
    // Only pure JS if no tsconfig
    if Path::new(root).join("tsconfig.json").exists() {
        return None;
    }
    let name = read_json_value(root, "package.json", "name");
    let version = read_json_value(root, "package.json", "version");
    let (pm, test_cmd, build_cmd) = detect_npm_tooling(root);
    let test = test_cmd.clone().unwrap_or_else(|| "npm test".into());
    let build = build_cmd.clone().unwrap_or_else(|| "npm run build".into());
    Some((pm, test_cmd, name, version, test, build))
}

fn detect_npm_tooling(root: &str) -> (String, Option<String>, Option<String>) {
    let lock_files = [
        ("pnpm-lock.yaml", "pnpm"),
        ("yarn.lock", "yarn"),
        ("package-lock.json", "npm"),
        ("bun.lockb", "bun"),
    ];
    for (file, tool) in &lock_files {
        if Path::new(root).join(file).exists() {
            let test = Some(format!("{} test", tool));
            let build = Some(format!("{} run build", tool));
            return (tool.to_string(), test, build);
        }
    }
    (
        "npm".into(),
        Some("npm test".into()),
        Some("npm run build".into()),
    )
}

fn detect_python(
    root: &str,
) -> Option<(
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    String,
)> {
    // Check for pyproject.toml with [build-system] first
    let pyproject = Path::new(root).join("pyproject.toml");
    if pyproject.exists() {
        let name = read_toml_value(root, "pyproject.toml", "project.name");
        let version = read_toml_value(root, "pyproject.toml", "project.version");
        let tool = if Path::new(root).join("poetry.lock").exists() {
            "poetry"
        } else if Path::new(root).join("uv.lock").exists() {
            "uv"
        } else {
            "pip"
        };
        let test = format!(
            "{} run pytest",
            if tool == "pip" { "python -m" } else { tool }
        );
        return Some((tool.into(), Some("pytest".into()), name, version, test));
    }

    if Path::new(root).join("setup.py").exists() {
        return Some((
            "pip".into(),
            Some("pytest".into()),
            None,
            None,
            "python -m pytest".into(),
        ));
    }

    if Path::new(root).join("requirements.txt").exists() {
        return Some((
            "pip".into(),
            Some("pytest".into()),
            None,
            None,
            "python -m pytest".into(),
        ));
    }

    None
}

fn detect_go(root: &str) -> Option<(Option<String>, Option<String>)> {
    if !Path::new(root).join("go.mod").exists() {
        return None;
    }
    let module = std::fs::read_to_string(Path::new(root).join("go.mod"))
        .ok()
        .and_then(|s| s.lines().next().map(|l| l.trim().to_string()))
        .filter(|l| l.starts_with("module "))
        .map(|l| l.trim_start_matches("module ").trim().to_string());
    Some((module, None))
}

fn detect_java(
    root: &str,
) -> Option<(
    String,
    String,
    Option<String>,
    Option<String>,
    String,
    String,
)> {
    if Path::new(root).join("pom.xml").exists() {
        Some((
            "maven".into(),
            "mvn test".into(),
            None,
            None,
            "mvn test".into(),
            "mvn package".into(),
        ))
    } else if Path::new(root).join("build.gradle").exists() {
        Some((
            "gradle".into(),
            "gradle test".into(),
            None,
            None,
            "gradle test".into(),
            "gradle build".into(),
        ))
    } else {
        None
    }
}

/// Detect language by counting source file extensions when no config files are found.
fn detect_by_extension(root: &str) -> Option<String> {
    use ignore::WalkBuilder;

    let mut counts: HashMap<String, usize> = HashMap::new();
    for entry in WalkBuilder::new(root)
        .hidden(true)
        .parents(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .max_depth(Some(3))
        .build()
        .filter_map(|e| e.ok())
    {
        if let Some(ext) = entry.path().extension().and_then(|e| e.to_str()) {
            if matches!(
                ext,
                "rs" | "py" | "ts" | "tsx" | "js" | "jsx" | "go" | "java"
            ) {
                *counts.entry(ext.to_string()).or_insert(0) += 1;
            }
        }
    }

    let lang_map = [
        ("rs", "Rust"),
        ("py", "Python"),
        ("ts", "TypeScript"),
        ("tsx", "TypeScript"),
        ("js", "JavaScript"),
        ("jsx", "JavaScript"),
        ("go", "Go"),
        ("java", "Java"),
    ];

    counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .and_then(|(ext, _)| {
            lang_map
                .iter()
                .find(|(e, _)| *e == ext)
                .map(|(_, lang)| lang.to_string())
        })
}

fn print_text(info: &ProjectInfo, verbose: bool) {
    println!("Language:   {}", info.language);
    if let Some(ref b) = info.build_system {
        println!("Build:      {}", b);
    }
    if let Some(ref t) = info.test_framework {
        println!("Test:       {}", t);
    }
    if let Some(ref l) = info.linter {
        println!("Lint:       {}", l);
    }
    if let Some(ref f) = info.formatter {
        println!("Format:     {}", f);
    }
    if let Some(ref n) = info.name {
        if let Some(ref v) = info.version {
            println!("Version:    {} v{}", n, v);
        } else {
            println!("Name:       {}", n);
        }
    }

    if verbose {
        println!("\nConfig files:");
        let mut keys: Vec<&String> = info.config_files.keys().collect();
        keys.sort();
        for key in keys {
            let status = if info.config_files[key] { "✓" } else { " " };
            println!("  [{}] {}", status, key);
        }

        if !info.scripts.is_empty() {
            println!("\nScripts:");
            let mut keys: Vec<&String> = info.scripts.keys().collect();
            keys.sort();
            for key in keys {
                println!("  {}: {}", key, info.scripts[key]);
            }
        }
    }
}
