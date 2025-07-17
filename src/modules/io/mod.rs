// File: src/modules/io/mod.rs
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_dicom<P: AsRef<Path>>(path: P) -> bool {
    let mut buffer = Vec::new();
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return false,
    };
    if file.read_to_end(&mut buffer).is_err() {
        return false;
    }
    let prefix_size = 128;
    if buffer.len() < prefix_size {
        return false;
    }
    let preamble = &buffer[prefix_size..prefix_size + 4];
    if preamble != b"DICM" {
        return false;
    }

    true
}
