use std::path::Path;
use std::path::PathBuf;
use std::fs::{create_dir_all, remove_dir_all, DirEntry, File};
use std::io::{self, Read};
use std::result::Result;
use std::env;
use std::process;

extern crate git2;
use git2::Repository;
use git2::Error as GitError;

extern crate bzip2;
use bzip2::write::BzEncoder;
use bzip2::Compression;

#[macro_use]
extern crate serde_derive;
extern crate toml;

#[derive(Deserialize)]
struct CargoConfig {
    package: Package
}

#[derive(Deserialize)]
struct Package {
    metadata: Metadata
}

#[derive(Deserialize)]
struct Metadata {
    #[serde(rename = "cldr-version")]
    cldr_version: String
}

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
const LOCALE_NAMES: &'static str = "localenames";
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

fn get_url_list() -> Vec<(String, &'static str)> {
    vec![
        (format!("{}-{}", URL_PREFIX, CORE), CORE),
        (format!("{}-{}-{}", URL_PREFIX, DATES, PACKAGE), DATES),
        (format!("{}-{}-{}", URL_PREFIX, LOCALE_NAMES, PACKAGE), LOCALE_NAMES),
        (format!("{}-{}-{}", URL_PREFIX, MISC, PACKAGE), MISC),
        (format!("{}-{}-{}", URL_PREFIX, NUMBERS, PACKAGE), NUMBERS),
        (format!("{}-{}", URL_PREFIX, RBNF), RBNF),
        (format!("{}-{}-{}", URL_PREFIX, SEGMENTS, PACKAGE), SEGMENTS),
        (format!("{}-{}-{}", URL_PREFIX, UNITS, PACKAGE), UNITS),
    ]
}

fn clone_repos() -> Result<(), GitError> {
    let json_path = Path::new("data/json");
    let tag = tag_name();
    for (url, dir) in get_url_list() {
        if json_path.join(dir).exists() {
            continue;
        }
        let repo = Repository::clone(url.as_ref(), json_path.join(dir))?;
        let ref_id = repo.refname_to_id(&tag)?;
        repo.set_head_detached(ref_id)?;
    }
    Ok(())
}

fn tag_name() -> String {
    let cargo_file = Path::new("Cargo.toml");
    let mut file = File::open(cargo_file).expect("couldn't open Cargo.toml");
    let mut s = String::new();
    file.read_to_string(&mut s).expect("Couldn't read Cargo.toml");
    let config: CargoConfig = toml::from_str(&s).expect("Couldn't deserialize Cargo.toml");
    format!("refs/tags/{}", config.package.metadata.cldr_version)
}

fn cleanup_json() {
    remove_dir_all("./data").unwrap();
}

fn visit_dirs<F>(dir: &Path, cb: &F) -> io::Result<()>
where
    F: Fn(&DirEntry) -> io::Result<()>,
{
    if dir.is_dir() {
        for entry in dir.read_dir()? {
            let entry = entry?;
            if entry.path().is_dir() {
                visit_dirs(&entry.path(), cb)?;
            } else {
                cb(&entry)?;
            }
        }
    }
    Ok(())
}

fn json_compressor(entry: &DirEntry) -> io::Result<()> {
    let entry_name = &entry.file_name();
    if is_json(&entry.path()) && !("bower.json" == entry_name || "package.json" == entry_name) {
        compress(&entry.path())?;
    }
    Ok(())
}

fn is_json(p: &Path) -> bool {
    if let Some(ex) = p.extension() {
        return ex == "json";
    }
    false
}

fn get_new_path(p: &Path) -> io::Result<PathBuf> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let p = match p.strip_prefix("data/json") {
        Ok(_p) => Path::new(&out_dir).join(_p).with_extension("json.bz2"),
        Err(_) => p.to_path_buf(),
    };
    if !p.exists() {
        create_dir_all(p.parent().unwrap())?;
    }
    Ok(p)
}

fn compress(p: &Path) -> io::Result<()> {
    let new_path = get_new_path(p)?;
    if new_path.exists() {
        return Ok(());
    }
    let mut old_file = File::open(&p)?;
    let new_file = File::create(&new_path)?;
    let mut zipper = BzEncoder::new(new_file, Compression::Best);
    io::copy(&mut old_file, &mut zipper)?;
    Ok(())
}

fn main() {
    if let Err(e) = clone_repos() {
        println!("Problem with CLDR repos: {}", e);
        println!("Cleaning up and exiting.");
        cleanup_json();
        process::exit(1);
    }
    if let Err(e) = visit_dirs(Path::new("data/json"), &json_compressor) {
        println!("Some problem: {}", e);
        cleanup_json();
        process::exit(1);
    }
    cleanup_json();
}
