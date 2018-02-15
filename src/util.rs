use std::{fs, path};
use std::io::Read;

use bzip2::read::BzDecoder;
use serde::de::DeserializeOwned;
use serde_json;

use error::Result;

pub fn read_cldr_data<P: AsRef<path::Path>, T: DeserializeOwned>(
    path: P,
    pointer: &str,
) -> Result<T> {
    let f = fs::File::open(path)?;
    let mut decompressor = BzDecoder::new(f);
    let mut contents = String::new();
    decompressor.read_to_string(&mut contents)?;
    let json: serde_json::value::Value = serde_json::from_str(&contents)?;
    let val = serde_json::from_value(
        json.pointer(pointer)
            .cloned()
            .unwrap_or(serde_json::Value::Null),
    )?;
    Ok(val)
}
