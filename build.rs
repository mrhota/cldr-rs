#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::path::Path;
use std::fs::File;
use std::io;
use std::io::Read;
use std::result::Result;

extern crate git2;
use git2::Repository;

extern crate serde;
extern crate serde_json as json;
use json::Value;
use json::value::to_value;

// For more information on the structure of the repos defined here, see
// https://github.com/unicode-cldr/cldr-json#package-organization
const URL_PREFIX: &'static str = "https://github.com/unicode-cldr/cldr";

// We have two kinds of packages for each functional batch of data: full and
// modern. "Modern" corresponds to locales in the CLDR subcommittee's modern
// coverage targets list, which likely satisfies most user's needs. "Full"
// includes all locales, including modern coverage targets.
#[cfg(feature = "all-modern")]
const PACKAGE: &'static str = "modern";
#[cfg(feature = "all-full")]
const PACKAGE: &'static str = "full";

// Core and rule-based number-formatting data don't have the full/modern split.
const CORE: &'static str = "core";
const RBNF: &'static str = "rbnf";

// These packages have the full/modern split.
const DATES: &'static str = "dates";
const LOCALENAMES: &'static str = "localenames";
const MISC: &'static str = "misc";
const NUMBERS: &'static str = "numbers";
const SEGMENTS: &'static str = "segments";
const UNITS: &'static str = "units";

// Calendars are also split, and there are lots of them.
const BUDDHIST: &'static str = "cal-buddhist";
const CHINESE: &'static str = "cal-chinese";
const COPTIC: &'static str = "cal-coptic";
const DANGI: &'static str = "cal-dangi";
const ETHIOPIC: &'static str = "cal-ethiopic";
const HEBREW: &'static str = "cal-hebrew";
const INDIAN: &'static str = "cal-indian";
const ISLAMIC: &'static str = "cal-islamic";
const JAPANESE: &'static str = "cal-japanese";
const PERSIAN: &'static str = "cal-persian";
const ROC: &'static str = "cal-roc";

fn build_url_list() -> Vec<(String, &'static str)> {
    vec![
        (format!("{}-{}", URL_PREFIX, CORE), CORE),
        (format!("{}-{}-{}", URL_PREFIX, DATES, PACKAGE), DATES),
        (format!("{}-{}-{}", URL_PREFIX, LOCALENAMES, PACKAGE), LOCALENAMES),
        (format!("{}-{}-{}", URL_PREFIX, MISC, PACKAGE), MISC),
        (format!("{}-{}-{}", URL_PREFIX, NUMBERS, PACKAGE), NUMBERS),
        (format!("{}-{}", URL_PREFIX, RBNF), RBNF),
        (format!("{}-{}-{}", URL_PREFIX, SEGMENTS, PACKAGE), SEGMENTS),
        (format!("{}-{}-{}", URL_PREFIX, UNITS, PACKAGE), UNITS)
    ]
}

fn clone_repos() {
    let json_path = Path::new("data/json");
    for (url, dir) in build_url_list() {
        if json_path.join(dir).exists() {
            continue;
        }
        println!("Cloning {}", url);
        let repo = match Repository::clone(url.as_ref(), json_path.join(dir)) {
            Ok(r) => r,
            Err(e) => panic!("couldn't clone repo: {}", e),
        };
    }
}

fn deserialize_json(p: &Path) -> Result<Value, io::Error> {
    let mut f = try!(File::open(p));
    let mut contents = String::new();
    try!(f.read_to_string(&mut contents));
    Ok(json::from_str(&contents).expect("couldn't deserialize to the provided type"))
}

fn main() {
    clone_repos();
    let core = Path::new("data/json/core");
    let available_locales = core.join("availableLocales.json");
    let default_content = core.join("defaultContent.json");
    let script_metadata = core.join("scriptMetadata.json");
    let dc_map: Value = deserialize_json(&available_locales).unwrap();
}
