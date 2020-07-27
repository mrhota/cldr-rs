use std::{fs::File, path::Path};
use std::io::Read;

use bzip2::read::BzDecoder;
use serde::de::DeserializeOwned;
use serde_json::{from_str, from_value, Value};

use error::Result;

pub fn read_cldr_data<P, T>(path: P, pointer: &str) -> Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let f = File::open(path)?;
    let mut decompressor = BzDecoder::new(f);
    let mut contents = String::new();
    decompressor.read_to_string(&mut contents)?;
    let json: Value = from_str(&contents)?;
    let val = from_value(json.pointer(pointer).cloned().unwrap_or(Value::Null))?;
    Ok(val)
}
