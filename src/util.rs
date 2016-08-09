#![allow(dead_code)]

use std::{fs, path};
use std::io::Read;

use error::Result;

use serde::de::Deserialize;
use serde_json;
use bzip2::read::BzDecoder;

pub fn read_cldr_data<P: AsRef<path::Path>, T: Deserialize>(path: P, pointer: &str) -> Result<T> {
    let f = try!(fs::File::open(path));
    let mut decompressor = BzDecoder::new(f);
    let mut contents = String::new();
    try!(decompressor.read_to_string(&mut contents));
    let json: serde_json::value::Value = try!(serde_json::from_str(&contents));
    let val = try!(serde_json::from_value(json.pointer(pointer)
                                              .cloned()
                                              .unwrap_or(serde_json::Value::Null)));
    Ok(val)
}
