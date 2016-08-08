#![allow(dead_code)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

extern crate serde;
extern crate serde_json;

pub mod aliases;

enum Accessor {
    AvailableLocales,
    ScriptMetadata(String),
}

impl Accessor {
    fn get(acc: Accessor) -> (PathBuf, String) {
        match acc {
            Accessor::AvailableLocales => {
                (Path::new("core").join("availableLocales"), "availableLocales".to_owned())
            },
            Accessor::ScriptMetadata(script) => {
                (Path::new("core").join("scriptMetadata"), format!("scriptMetadata.{}", script))
            },
        }
    }
}

#[derive(Deserialize, Default)]
pub struct AvailableLocales {
    modern: Vec<String>,
    full: Vec<String>
}

pub type DefaultContent = Vec<String>;

#[derive(Deserialize, Default)]
pub struct ScriptMetadata {
    scripts: BTreeMap<String, _ScriptMetadata>
}

#[derive(Deserialize, Default)]
pub struct _Version {
    m_version_: i32
}

/// This metadata comes from the `common/properties/scriptMetadata.txt`
/// file in a CLDR distribution; that is, it isn't part of the LDML files.
#[derive(Deserialize, Default)]
pub struct _ScriptMetadata {
    rank: u16,
    age: _Version,
    #[serde(rename = "sampleChar")]
    sample_char: char,
    #[serde(rename = "idUsage")]
    id_usage: String,
    rtl: String,
    #[serde(rename = "lbLetters")]
    lb_letters: String,
    #[serde(rename = "hasCase")]
    has_case: String,
    #[serde(rename = "shapingReq")]
    shaping_req: String,
    ime: String,
    density: i16,
    #[serde(rename = "originCountry")]
    origin_country: String,
    #[serde(rename = "likelyLanguage")]
    likely_language: String
}
