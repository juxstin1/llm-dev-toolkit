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

    if name.ends_with(".zip") {
        extract_zip(archive_path, &output_dir)
    } else if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
        extract_tar_gz(archive_path, &output_dir)
    } else if name.ends_with(".tar") {
        extract_tar(archive_path, &output_dir)
    } else if name.ends_with(".gz") {
        extract_gz(archive_path, &output_dir)
    } else {
        Err(format!("Unsupported archive format: {}", name))
    }
}

fn extract_zip(path: &Path, output_dir: &str) -> Result<(), String> {
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
    archive.extract(output_dir).map_err(|e| e.to_string())?;
    println!("Extracted {} entries to {}", archive.len(), output_dir);
    Ok(())
}

fn extract_tar_gz(path: &Path, output_dir: &str) -> Result<(), String> {
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);
    archive.unpack(output_dir).map_err(|e| e.to_string())?;
    let entries = fs::read_dir(output_dir).map_err(|e| e.to_string())?;
    let count = entries.filter_map(|e| e.ok()).count();
    println!("Extracted {} entries to {}", count, output_dir);
    Ok(())
}

fn extract_tar(path: &Path, output_dir: &str) -> Result<(), String> {
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut archive = tar::Archive::new(file);
    archive.unpack(output_dir).map_err(|e| e.to_string())?;
    let entries = fs::read_dir(output_dir).map_err(|e| e.to_string())?;
    let count = entries.filter_map(|e| e.ok()).count();
    println!("Extracted {} entries to {}", count, output_dir);
    Ok(())
}

fn extract_gz(path: &Path, output_dir: &str) -> Result<(), String> {
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut decoder = flate2::read::GzDecoder::new(file);
    let out_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let out_path = Path::new(output_dir).join(out_name);
    let mut out_file = fs::File::create(&out_path).map_err(|e| e.to_string())?;
    std::io::copy(&mut decoder, &mut out_file).map_err(|e| e.to_string())?;
    println!("Extracted to {}", out_path.display());
    Ok(())
}
