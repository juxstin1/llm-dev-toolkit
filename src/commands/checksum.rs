use std::fs::File;

use md5::Md5;
use rayon::prelude::*;
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512};

use super::with_rayon;
use crate::ChecksumArgs;

fn checksum_file(path: &str, algorithm: &str) -> Result<String, String> {
    let mut file = File::open(path).map_err(|e| format!("{}: {}", path, e))?;

    let hash: String = match algorithm {
        "sha256" | "" => {
            let mut hasher = Sha256::new();
            std::io::copy(&mut file, &mut hasher).map_err(|e| format!("{}: {}", path, e))?;
            format!("{:x}", hasher.finalize())
        }
        "sha224" => {
            let mut hasher = Sha224::new();
            std::io::copy(&mut file, &mut hasher).map_err(|e| format!("{}: {}", path, e))?;
            format!("{:x}", hasher.finalize())
        }
        "sha384" => {
            let mut hasher = Sha384::new();
            std::io::copy(&mut file, &mut hasher).map_err(|e| format!("{}: {}", path, e))?;
            format!("{:x}", hasher.finalize())
        }
        "sha512" => {
            let mut hasher = Sha512::new();
            std::io::copy(&mut file, &mut hasher).map_err(|e| format!("{}: {}", path, e))?;
            format!("{:x}", hasher.finalize())
        }
        "md5" => {
            let mut hasher = Md5::new();
            std::io::copy(&mut file, &mut hasher).map_err(|e| format!("{}: {}", path, e))?;
            format!("{:x}", hasher.finalize())
        }
        _ => return Err(format!("Unsupported algorithm: {}", algorithm)),
    };

    Ok(hash)
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

    let results: Vec<(&String, Result<String, String>)> = with_rayon(args.threads, || {
        args.files
            .par_iter()
            .map(|path| (path, checksum_file(path, algorithm)))
            .collect()
    });

    for (path, result) in results {
        match result {
            Ok(hash) => println!("{}  {}", hash, path),
            Err(e) => eprintln!("{}", e),
        }
    }

    Ok(())
}
