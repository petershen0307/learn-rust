pub mod green_thread;

use sha2::{Digest, Sha512};
use walkdir::WalkDir;

fn file_sha512(file_reader: &mut impl std::io::Read) -> String {
    let mut hasher = Sha512::new();
    std::io::copy(file_reader, &mut hasher).unwrap();
    let hash_bytes = hasher.finalize();
    base16ct::lower::encode_string(&hash_bytes)
}

#[test]
fn test_sha512() {
    let mut reader = std::io::BufReader::new("1234".as_bytes());
    let file_sha = file_sha512(&mut reader);
    assert_eq!(String::from("d404559f602eab6fd602ac7680dacbfaadd13630335e951f097af3900e9de176b6db28512f2e000b9d04fba5133e8b1c6e8df59db3a8ab9d60be4b97cc9e81db"),file_sha);
}

pub fn list_files_sha512(path: &std::path::Path) -> Vec<String> {
    let mut result = Vec::new();
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() {
            // skip dir
            continue;
        }
        let mut file = std::fs::File::open(entry.path()).unwrap();
        let sha = file_sha512(&mut file);
        result.push(format!("{} {}", entry.path().display(), sha))
    }
    result.sort();
    result
}

#[test]
fn test_list_files() {
    let r = list_files_sha512(std::path::Path::new("./src/bin"));
    println!("{}", r.join("\n"));
}
