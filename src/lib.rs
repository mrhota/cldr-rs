#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;

#[derive(Deserialize)]
pub struct AvailableLocales {
    pub modern: Vec<String>,
    pub full: Vec<String>
}
