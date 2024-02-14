use sha2::{Digest, Sha512};

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
