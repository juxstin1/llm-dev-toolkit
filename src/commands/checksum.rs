use std::path::Path;

use md5::Md5;
use rayon::prelude::*;
use serde::Serialize;
use sha2::{Sha224, Sha256, Sha384, Sha512};

use super::{hash_file, with_rayon};
use crate::ChecksumArgs;

#[derive(Serialize)]
struct Checksum {
    path: String,
    algorithm: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

fn checksum_file(path: &str, algorithm: &str) -> Result<String, String> {
    let p = Path::new(path);
    match algorithm {
        "sha256" | "" => hash_file::<Sha256>(p),
        "sha224" => hash_file::<Sha224>(p),
        "sha384" => hash_file::<Sha384>(p),
        "sha512" => hash_file::<Sha512>(p),
        "md5" => hash_file::<Md5>(p),
        _ => Err(format!("Unsupported algorithm: {}", algorithm)),
    }
}

fn validate_algorithm(algorithm: &str) -> Result<(), String> {
    match algorithm {
        "sha256" | "sha224" | "sha384" | "sha512" | "md5" => Ok(()),
        _ => Err(format!("Unsupported algorithm: {}", algorithm)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum_sha256() {
        let tmp = std::env::temp_dir().join("tk-test-checksum-sha256.txt");
        std::fs::write(&tmp, b"hello world\n").unwrap();
        let hash = checksum_file(&tmp.to_string_lossy(), "sha256").unwrap();
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
        std::fs::remove_file(&tmp).unwrap();
    }

    #[test]
    fn test_checksum_md5() {
        let tmp = std::env::temp_dir().join("tk-test-checksum-md5.txt");
        std::fs::write(&tmp, b"hello world\n").unwrap();
        let hash = checksum_file(&tmp.to_string_lossy(), "md5").unwrap();
        assert_eq!(hash.len(), 32);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
        std::fs::remove_file(&tmp).unwrap();
    }

    #[test]
    fn test_checksum_unsupported_algorithm() {
        let tmp = std::env::temp_dir().join("tk-test-checksum-bad.txt");
        std::fs::write(&tmp, b"hello").unwrap();
        let result = checksum_file(&tmp.to_string_lossy(), "sha1");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported algorithm"));
        std::fs::remove_file(&tmp).unwrap();
    }
}

pub fn run(args: &ChecksumArgs) -> Result<(), String> {
    let algorithm = if args.algorithm.is_empty() {
        "sha256"
    } else {
        &args.algorithm
    };
    validate_algorithm(algorithm)?;

    let results: Vec<(&String, Result<String, String>)> = with_rayon(args.threads, || {
        args.files
            .par_iter()
            .map(|path| (path, checksum_file(path, algorithm)))
            .collect()
    });

    if super::json_enabled() {
        let out: Vec<Checksum> = results
            .into_iter()
            .map(|(path, result)| match result {
                Ok(hash) => Checksum {
                    path: path.clone(),
                    algorithm: algorithm.to_string(),
                    hash: Some(hash),
                    error: None,
                },
                Err(e) => Checksum {
                    path: path.clone(),
                    algorithm: algorithm.to_string(),
                    hash: None,
                    error: Some(e),
                },
            })
            .collect();
        return super::emit_json(&out);
    }

    for (path, result) in results {
        match result {
            Ok(hash) => println!("{}  {}", hash, path),
            Err(e) => eprintln!("{}", e),
        }
    }

    Ok(())
}
