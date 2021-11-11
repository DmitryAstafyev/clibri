use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::io::{BufReader, Read};
use std::{fs::File, path::Path};

pub fn get(filename: &Path) -> Result<String, String> {
    let input = File::open(filename).map_err(|e| e.to_string())?;
    let reader = BufReader::new(input);
    let digest = calc_sha256_digest(reader)?;
    Ok(HEXUPPER.encode(digest.as_ref()))
}

fn calc_sha256_digest<R: Read>(mut reader: R) -> Result<Digest, String> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];
    loop {
        let count = reader.read(&mut buffer).map_err(|e| e.to_string())?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }
    Ok(context.finish())
}
