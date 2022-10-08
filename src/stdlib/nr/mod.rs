use std::io;
use std::io::{Read};
use flate2::read::GzEncoder;
use flate2::Compression;

pub mod graphql;
pub mod event;

pub fn compress_payload(payload: &String) -> io::Result<Vec<u8>> {
    let mut result = Vec::new();
    let mut z = GzEncoder::new(&payload.as_bytes()[..], Compression::fast());
    z.read_to_end(&mut result)?;
    Ok(result)
}
