use std::fs;
use std::path::{Path, PathBuf};

fn system_info() -> Result<(), String> {
    let os = std::env::consts::OS;
    let cpus = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let current_dir = std::env::current_dir()
        .map_err(|e| e.to_string())?
        .to_string_lossy()
        .to_string();
    let home = if cfg!(target_os = "windows") {
        std::env::var("USERPROFILE").unwrap_or_default()
    } else {
        std::env::var("HOME").unwrap_or_default()
    };
    let disk_usage = dir_size(".")?;

    println!("OS:              {}", os);
    println!("CPU Cores:       {}", cpus);
    println!("Current Dir:     {}", current_dir);
    println!("Home Dir:        {}", home);
    println!(
        "Disk Usage:      {} (current dir)",
        crate::commands::format_size(disk_usage)
    );

    Ok(())
}

fn file_info(path: &str) -> Result<(), String> {
    let meta = fs::metadata(path).map_err(|e| format!("Cannot access '{}': {}", path, e))?;
    let p = Path::new(path);

    let size = meta.len();
    let modified = meta.modified().ok().and_then(|t| {
        t.duration_since(std::time::UNIX_EPOCH)
            .ok()
            .map(|d| d.as_secs() as i64)
    });
    let created = meta.created().ok().and_then(|t| {
        t.duration_since(std::time::UNIX_EPOCH)
            .ok()
            .map(|d| d.as_secs() as i64)
    });

    let file_type = if meta.is_dir() {
        "directory"
    } else if meta.is_symlink() {
        "symlink"
    } else {
        "file"
    };

    let ext = p
        .extension()
        .map(|e| e.to_string_lossy().to_string())
        .unwrap_or_default();

    let mime = mime_guess::from_path(path)
        .first_or_octet_stream()
        .to_string();

    println!(
        "File:            {}",
        p.file_name().unwrap_or_default().to_string_lossy()
    );
    println!("Path:            {}", path);
    println!("Size:            {}", crate::commands::format_size(size));
    println!("Type:            {}", file_type);
    println!("Extension:       .{}", ext);
    println!("MIME:            {}", mime);

    if let Some(t) = modified {
        println!("Modified:        {}", crate::commands::format_time(t));
    }
    if let Some(t) = created {
        println!("Created:         {}", crate::commands::format_time(t));
    }

    let perms = meta.permissions();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = perms.mode();
        println!("Permissions:     {:o}", mode & 0o777);
    }
    #[cfg(not(unix))]
    {
        println!("Read-only:       {}", perms.readonly());
    }

    Ok(())
}

/// Sum of regular-file sizes under `path`.
///
/// Iterative (an explicit stack rather than recursion) so deep trees can't
/// overflow the stack, and uses `symlink_metadata` so directory symlinks are
/// never followed — that avoids both cycles and double-counting. Unreadable
/// subdirectories are skipped rather than aborting the whole walk.
fn dir_size(path: &str) -> Result<u64, String> {
    let mut total = 0u64;
    let mut stack = vec![PathBuf::from(path)];

    while let Some(dir) = stack.pop() {
        let rd = match fs::read_dir(&dir) {
            Ok(rd) => rd,
            Err(_) => continue,
        };
        for entry in rd.flatten() {
            let meta = match entry.path().symlink_metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };
            let ft = meta.file_type();
            if ft.is_symlink() {
                continue;
            } else if ft.is_dir() {
                stack.push(entry.path());
            } else {
                total += meta.len();
            }
        }
    }

    Ok(total)
}

pub fn run(args: &crate::InfoArgs) -> Result<(), String> {
    match &args.file {
        Some(path) => file_info(path),
        None => system_info(),
    }
}
