use std::fs;
use std::path::Path;

pub fn run(args: &crate::ExtractArgs) -> Result<(), String> {
    let archive_path = Path::new(&args.archive);
    let output_dir = args.output.clone().unwrap_or_else(|| {
        archive_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_string_lossy()
            .to_string()
    });

    fs::create_dir_all(&output_dir).map_err(|e| e.to_string())?;

    let name = archive_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    let count = if name.ends_with(".zip") {
        extract_zip(archive_path, &output_dir)?
    } else if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
        extract_tar_gz(archive_path, &output_dir)?
    } else if name.ends_with(".tar") {
        extract_tar(archive_path, &output_dir)?
    } else if name.ends_with(".gz") {
        extract_gz(archive_path, &output_dir)?
    } else {
        return Err(format!("Unsupported archive format: {}", name));
    };

    if crate::commands::json_enabled() {
        return crate::commands::emit_json(&serde_json::json!({
            "archive": args.archive,
            "output_dir": output_dir,
            "entries": count,
        }));
    }

    let entry_word = if count == 1 { "entry" } else { "entries" };
    println!("Extracted {} {} to {}", count, entry_word, output_dir);
    Ok(())
}

fn extract_zip(path: &Path, output_dir: &str) -> Result<usize, String> {
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
    let out_root = Path::new(output_dir);
    let mut count = 0usize;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(|e| e.to_string())?;

        // `enclosed_name` rejects absolute paths and any `..` that would escape
        // the archive root, returning None — the primary zip-slip guard.
        let rel = match entry.enclosed_name() {
            Some(name) => name,
            None => return Err(format!("unsafe path in archive: {}", entry.name())),
        };
        let dest = out_root.join(&rel);

        // Defense in depth: the resolved path must stay under the output dir.
        if !dest.starts_with(out_root) {
            return Err(format!("path escapes output dir: {}", entry.name()));
        }

        if entry.is_dir() {
            fs::create_dir_all(&dest).map_err(|e| e.to_string())?;
        } else {
            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            let mut out = fs::File::create(&dest).map_err(|e| e.to_string())?;
            std::io::copy(&mut entry, &mut out).map_err(|e| e.to_string())?;
            count += 1;
        }
    }

    Ok(count)
}

fn extract_tar_gz(path: &Path, output_dir: &str) -> Result<usize, String> {
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);
    archive.unpack(output_dir).map_err(|e| e.to_string())?;
    let entries = fs::read_dir(output_dir).map_err(|e| e.to_string())?;
    Ok(entries.filter_map(|e| e.ok()).count())
}

fn extract_tar(path: &Path, output_dir: &str) -> Result<usize, String> {
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut archive = tar::Archive::new(file);
    archive.unpack(output_dir).map_err(|e| e.to_string())?;
    let entries = fs::read_dir(output_dir).map_err(|e| e.to_string())?;
    Ok(entries.filter_map(|e| e.ok()).count())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_extract_zip_rejects_traversal() {
        let dir = std::env::temp_dir().join("tk-zipslip-test");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        // Build a zip whose entry tries to escape the extraction root.
        let zip_path = dir.join("evil.zip");
        {
            let f = fs::File::create(&zip_path).unwrap();
            let mut zw = zip::ZipWriter::new(f);
            let opts = zip::write::SimpleFileOptions::default();
            zw.start_file("../escaped.txt", opts).unwrap();
            zw.write_all(b"pwned").unwrap();
            zw.finish().unwrap();
        }

        let out = dir.join("out");
        fs::create_dir_all(&out).unwrap();
        let result = extract_zip(&zip_path, &out.to_string_lossy());

        assert!(result.is_err(), "traversal entry must be rejected");
        assert!(
            !dir.join("escaped.txt").exists(),
            "escape target must not be written outside the output dir"
        );

        let _ = fs::remove_dir_all(&dir);
    }
}

fn extract_gz(path: &Path, output_dir: &str) -> Result<usize, String> {
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut decoder = flate2::read::GzDecoder::new(file);
    let out_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let out_path = Path::new(output_dir).join(out_name);
    let mut out_file = fs::File::create(&out_path).map_err(|e| e.to_string())?;
    std::io::copy(&mut decoder, &mut out_file).map_err(|e| e.to_string())?;
    Ok(1)
}
