#![allow(clippy::items_after_test_module)]

mod commands;
mod mcp;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tk", version, about = "LLM Dev Toolkit", term_width = 100)]
struct Cli {
    #[arg(
        long,
        global = true,
        default_value = "auto",
        value_enum,
        help = "Color output: auto, always, never"
    )]
    color: commands::ColorChoice,
    #[arg(
        long,
        global = true,
        default_value = "text",
        value_enum,
        help = "Output format: text (human) or json (machine-readable)"
    )]
    format: commands::OutputFormat,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(alias = "l", about = "List directory contents")]
    Ls(LsArgs),
    #[command(about = "Short for 'ls -a'")]
    La(LaArgs),
    #[command(about = "Short for 'ls -al'")]
    Ll(LlArgs),
    #[command(about = "Display directory tree with depth limit")]
    Ltd(LtdArgs),
    #[command(about = "Find files by name (substring match)")]
    Ff(FfArgs),
    #[command(
        alias = "find",
        about = "Find files by name (substring match, alias for ff)"
    )]
    Fd(FdArgs),
    #[command(about = "Find files by extension")]
    FfExt(FfExtArgs),
    #[command(about = "Find files by substring match in name")]
    FfName(FfNameArgs),
    #[command(alias = "grep", about = "Search file contents (grep-like patterns)")]
    Search(SearchArgs),
    #[command(about = "Concatenate and display files")]
    Cat(CatArgs),
    #[command(about = "Syntax-highlighted file preview")]
    Preview(PreviewArgs),
    #[command(about = "Display first lines of files")]
    Head(HeadArgs),
    #[command(about = "Display last lines of files")]
    Tail(TailArgs),
    #[command(about = "Show file/directory statistics")]
    Stats(StatsArgs),
    #[command(about = "Find duplicate files by SHA-256 hash")]
    Dups(DupsArgs),
    #[command(about = "List recently modified files")]
    Recent(RecentArgs),
    #[command(about = "Show largest files or directories")]
    Largest(LargestArgs),
    #[command(about = "Find empty files and directories")]
    Empty(EmptyArgs),
    #[command(alias = "lt", about = "Display directory tree")]
    Tree(TreeArgs),
    #[command(about = "Read/write system clipboard")]
    Clip(ClipArgs),
    #[command(about = "Count lines, words, chars, bytes")]
    Count(CountArgs),
    #[command(about = "Compute file checksums (SHA-256 default)")]
    Checksum(ChecksumArgs),
    #[command(about = "Extract archives (zip, tar, gz)")]
    Extract(ExtractArgs),
    #[command(about = "Format, validate, or inspect JSON")]
    Json(JsonArgs),
    #[command(about = "Sort directory entries by various criteria")]
    Sort(SortArgs),
    #[command(about = "Show file info or system overview")]
    Info(InfoArgs),
    #[command(about = "Run as an MCP server over stdio (read-only tools for LLM agents)")]
    Mcp,
}

#[derive(clap::Args)]
struct LsArgs {
    path: Option<String>,
    #[arg(short = 'a', long, help = "Show hidden entries")]
    all: bool,
    #[arg(short = 'l', long, help = "Long format with permissions and sizes")]
    long: bool,
}

type LaArgs = LsArgs;
type LlArgs = LsArgs;

type LtdArgs = LtdArgsInner;

#[derive(clap::Args)]
struct LtdArgsInner {
    #[arg(short = 'L', long, help = "Maximum depth to display")]
    depth: Option<usize>,
    path: Option<String>,
}

#[derive(clap::Args)]
struct FfArgs {
    pattern: String,
    path: Option<String>,
    #[arg(short = 'i', long, help = "Case-insensitive matching")]
    ignore_case: bool,
    #[arg(short = 'e', long, help = "Filter by file extension (e.g. rs)")]
    ext: Option<String>,
    #[arg(
        short = 't',
        long,
        help = "Filter by type: 'f' for files, 'd' for dirs"
    )]
    type_filter: Option<String>,
}

type FdArgs = FfArgs;

#[derive(clap::Args)]
struct FfExtArgs {
    ext: String,
    path: Option<String>,
}

#[derive(clap::Args)]
struct FfNameArgs {
    pattern: String,
    path: Option<String>,
    #[arg(short = 'i', long, help = "Case-insensitive matching")]
    ignore_case: bool,
    #[arg(
        short = 'g',
        long,
        help = "Use glob pattern matching instead of substring"
    )]
    glob: bool,
}

#[derive(clap::Args)]
struct SearchArgs {
    pattern: String,
    path: Option<String>,
    #[arg(short = 'i', long, help = "Case-insensitive matching")]
    ignore_case: bool,
    #[arg(long, help = "Show line numbers")]
    line_number: bool,
    #[arg(short = 'C', long, help = "Show N lines of context around matches")]
    context: Option<usize>,
    #[arg(short = 'l', long, help = "Only print filenames with matches")]
    files_with_matches: bool,
    #[arg(short = 'e', long, help = "Filter by file extension (e.g. rs)")]
    ext: Option<String>,
}

#[derive(clap::Args)]
struct CatArgs {
    files: Vec<String>,
    #[arg(short = 'n', long, help = "Number output lines")]
    number: bool,
}

#[derive(clap::Args)]
struct PreviewArgs {
    files: Vec<String>,
    #[arg(short = 'l', long, help = "Force syntax highlighting language")]
    language: Option<String>,
    #[arg(short = 'n', long, help = "Show line numbers")]
    number: bool,
}

#[derive(clap::Args)]
struct HeadArgs {
    files: Vec<String>,
    #[arg(
        short = 'n',
        long,
        default_value = "10",
        help = "Number of lines to show"
    )]
    lines: usize,
}

type TailArgs = HeadArgs;

#[derive(clap::Args)]
struct StatsArgs {
    path: Option<String>,
    #[arg(short = 'd', long, help = "Show per-directory breakdown")]
    directory: bool,
    #[arg(short = 't', long, help = "Show per-extension breakdown")]
    by_type: bool,
    #[arg(long, help = "Maximum directory depth")]
    max_depth: Option<usize>,
}

#[derive(clap::Args)]
struct DupsArgs {
    path: Option<String>,
    #[arg(short = 'm', long, help = "Minimum file size (e.g. 1kib, 1mb)")]
    min_size: Option<String>,
    #[arg(short = 'd', long, help = "Prompt before deleting duplicates")]
    delete: bool,
    #[arg(long, help = "Number of parallel hash threads (default: CPU count)")]
    threads: Option<usize>,
}

#[derive(clap::Args)]
struct RecentArgs {
    path: Option<String>,
    #[arg(
        short = 'n',
        long,
        default_value = "20",
        help = "Number of files to show"
    )]
    count: usize,
    #[arg(
        short = 'd',
        long,
        default_value = "7",
        help = "How far back to look (in days)"
    )]
    days: u64,
    #[arg(short = 'e', long, help = "Filter by extension (e.g. rs)")]
    ext: Option<String>,
}

#[derive(clap::Args)]
struct LargestArgs {
    path: Option<String>,
    #[arg(
        short = 'n',
        long,
        default_value = "20",
        help = "Number of entries to show"
    )]
    count: usize,
    #[arg(short = 'd', long, help = "Show largest directories instead of files")]
    directories: bool,
}

#[derive(clap::Args)]
struct EmptyArgs {
    path: Option<String>,
    #[arg(short = 'f', long, help = "Show empty files only")]
    files: bool,
    #[arg(short = 'd', long, help = "Show empty directories only")]
    dirs: bool,
}

#[derive(clap::Args)]
struct TreeArgs {
    path: Option<String>,
    #[arg(short = 'L', long, help = "Maximum depth to display")]
    depth: Option<usize>,
    #[arg(short = 'a', long, help = "Include hidden entries")]
    all: bool,
    #[arg(short = 'd', long, help = "Show directories only")]
    dirs_only: bool,
}

#[derive(clap::Args)]
struct ClipArgs {
    #[arg(short = 'o', long, help = "Write clipboard contents to stdout")]
    out: bool,
    #[arg(short = 'i', long, help = "Read stdin into clipboard")]
    r#in: bool,
    value: Option<String>,
}

#[derive(clap::Args)]
struct CountArgs {
    files: Vec<String>,
    #[arg(short = 'w', long, help = "Count words")]
    words: bool,
    #[arg(short = 'c', long, help = "Count characters")]
    chars: bool,
    #[arg(short = 'l', long, help = "Count lines")]
    lines: bool,
    #[arg(short = 'b', long, help = "Count bytes (raw)")]
    bytes: bool,
}

#[derive(clap::Args)]
struct ChecksumArgs {
    files: Vec<String>,
    #[arg(
        short = 'a',
        long,
        default_value = "sha256",
        help = "Hash algorithm: sha256, sha224, sha384, sha512, md5"
    )]
    algorithm: String,
    #[arg(long, help = "Number of parallel hash threads (default: CPU count)")]
    threads: Option<usize>,
}

#[derive(clap::Args)]
struct ExtractArgs {
    archive: String,
    #[arg(
        short = 'o',
        long,
        help = "Output directory (default: same as archive)"
    )]
    output: Option<String>,
}

#[derive(clap::Args)]
struct JsonArgs {
    #[command(subcommand)]
    action: JsonAction,
}

#[derive(Subcommand)]
enum JsonAction {
    #[command(about = "Pretty-print JSON")]
    Format {
        file: Option<String>,
        #[arg(short = 's', long, help = "Indentation spaces per level")]
        spaces: Option<usize>,
    },
    #[command(about = "Validate JSON syntax")]
    Validate { file: Option<String> },
    #[command(about = "List top-level keys")]
    Keys { file: Option<String> },
}

#[derive(clap::Args)]
struct SortArgs {
    path: Option<String>,
    #[arg(
        short = 'b',
        long,
        default_value = "name",
        help = "Sort field: name, size, date, ext"
    )]
    by: String,
    #[arg(short = 'r', long, help = "Reverse sort order")]
    reverse: bool,
    #[arg(
        short = 'n',
        long,
        default_value = "50",
        help = "Number of entries to show"
    )]
    count: usize,
    #[arg(short = 'd', long, help = "List directories before files")]
    dirs_first: bool,
}

#[derive(clap::Args)]
struct InfoArgs {
    #[arg(short = 'f', long, help = "Path to a specific file")]
    file: Option<String>,
}

/// Exit quietly on a broken pipe (e.g. `tk tree | head`) instead of letting the
/// `println!` write error surface as a panic. There is no stable std API to reset
/// the SIGPIPE handler, and on Windows the failure is an I/O error rather than a
/// signal, so intercepting the panic covers both platforms without extra deps.
fn install_broken_pipe_guard() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let msg = info
            .payload()
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| info.payload().downcast_ref::<&str>().copied())
            .unwrap_or("");
        let broken_pipe = msg.contains("Broken pipe")
            || msg.contains("os error 232") // Windows ERROR_NO_DATA
            || msg.contains("pipe is being closed");
        if broken_pipe {
            std::process::exit(0);
        }
        default_hook(info);
    }));
}

fn main() {
    install_broken_pipe_guard();

    let cli = Cli::parse();

    commands::init_format(cli.format);
    commands::init_color(cli.color);

    let result = match &cli.command {
        Commands::Ls(a) => commands::ls::run(a),
        Commands::La(a) => commands::ls::run_all(a),
        Commands::Ll(a) => commands::ls::run_long(a),
        Commands::Ltd(a) => commands::tree::run_depth(a),
        Commands::Ff(a) => commands::find::run_name(a),
        Commands::Fd(a) => commands::find::run_name(a),
        Commands::FfExt(a) => commands::find::run_ext(a),
        Commands::FfName(a) => commands::find::run_name_pattern(a),
        Commands::Search(a) => commands::search::run(a),
        Commands::Cat(a) => commands::view::run_cat(a),
        Commands::Preview(a) => commands::view::run_preview(a),
        Commands::Head(a) => commands::view::run_head(a),
        Commands::Tail(a) => commands::view::run_tail(a),
        Commands::Stats(a) => commands::stats::run(a),
        Commands::Dups(a) => commands::dups::run(a),
        Commands::Recent(a) => commands::recent::run(a),
        Commands::Largest(a) => commands::largest::run(a),
        Commands::Empty(a) => commands::empty::run(a),
        Commands::Tree(a) => commands::tree::run(a),
        Commands::Clip(a) => commands::clip::run(a),
        Commands::Count(a) => commands::count::run(a),
        Commands::Checksum(a) => commands::checksum::run(a),
        Commands::Extract(a) => commands::extract::run(a),
        Commands::Json(a) => commands::json::run(a),
        Commands::Sort(a) => commands::sort::run(a),
        Commands::Info(a) => commands::info::run(a),
        Commands::Mcp => mcp::run(),
    };

    if let Err(e) = result {
        // Keep the error contract uniform with --format json: a structured
        // consumer gets a parseable `{"error": ...}` on stderr (stdout stays
        // empty), distinguished from success by the non-zero exit code.
        if commands::json_enabled() {
            eprintln!("{}", serde_json::json!({ "error": e }));
        } else {
            eprintln!("Error: {}", e);
        }
        std::process::exit(1);
    }
}
