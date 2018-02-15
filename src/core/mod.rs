use std::path::Path;
use error::Result;

use util::read_cldr_data;

extern crate serde;
extern crate serde_json;
use serde::de::DeserializeOwned;

pub mod aliases;

const OUT: &str = env!("OUT_DIR");

/// Use these variants to request instances of various kinds of CLDR data.
/// Variants with strings expect BCP 47-like language tags.
pub enum Access<'a> {
    AvailableLocales,
    ScriptMetadata(&'a str),
    DefaultContent,
}

impl<'a> Access<'a> {
    pub fn access<T: DeserializeOwned>(&self) -> Result<T> {
        match *self {
            Access::AvailableLocales => read_cldr_data(
                Path::new(OUT)
                    .join("core")
                    .join("availableLocales.json.bz2"),
                "/availableLocales",
            ),
            Access::ScriptMetadata(script) => read_cldr_data(
                Path::new(OUT).join("core").join("scriptMetadata.json.bz2"),
                &format!("/scriptMetadata/{}", script),
            ),
            Access::DefaultContent => read_cldr_data(
                Path::new(OUT).join("core").join("defaultContent.json.bz2"),
                "/defaultContent",
            ),
        }
    }
}

#[derive(Deserialize, Default)]
pub struct AvailableLocales {
    pub modern: Vec<String>,
    pub full: Vec<String>,
}

pub type DefaultContent = Vec<String>;

#[derive(Deserialize, Default)]
pub struct _Version {
    pub m_version_: i32,
}

/// This metadata derives from the `common/properties/scriptMetadata.txt`
/// file in a CLDR distribution. The LDML does not define or describe this
/// data, but users might find it interesting or useful.
#[derive(Deserialize, Default)]
pub struct ScriptMetadata {
    pub rank: u16,
    pub age: _Version,
    #[serde(rename = "sampleChar")]
    pub sample_char: char,
    #[serde(rename = "idUsage")]
    pub id_usage: String,
    pub rtl: String,
    #[serde(rename = "lbLetters")]
    pub lb_letters: String,
    #[serde(rename = "hasCase")]
    pub has_case: String,
    #[serde(rename = "shapingReq")]
    pub shaping_req: String,
    pub ime: String,
    pub density: i16,
    #[serde(rename = "originCountry")]
    pub origin_country: String,
    #[serde(rename = "likelyLanguage")]
    pub likely_language: String,
}
