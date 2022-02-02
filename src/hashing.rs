const TESTFILE: &[u8] = include_bytes!("../assets/embededfile.txt");

pub fn test_xxhash3() {
    let hash = xxhash_rust::xxh3::xxh3_64(TESTFILE);
    println!("xxh3_64: {hash:16x}");
}

pub fn test_crc32fast() {
    let hash = crc32fast::hash(TESTFILE);
    println!("CRC32:   {hash:8x}");
}

pub fn run() {
    test_xxhash3();
    test_crc32fast();
}