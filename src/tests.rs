#![cfg(test)]

use std::borrow::Cow;
use core::*;

#[test]
fn reading_data_works() {
    let item: AvailableLocales = Access::AvailableLocales.access().unwrap();
    assert!(item.modern.iter().any(|s| s == "zh-Hans"));
}

#[test]
fn reading_unicode_works() {
    let item1: ScriptMetadata = Access::ScriptMetadata(Cow::from("Hans")).access().unwrap();
    let item2: ScriptMetadata = Access::ScriptMetadata(Cow::from("Hebr")).access().unwrap();
    assert_eq!(item1.sample_char, '字');
    assert_eq!(item2.sample_char, 'א');
}

#[test]
#[should_panic]
fn reading_nonexistent_script_fails() {
    Access::ScriptMetadata(Cow::from("Hurdur")).access::<ScriptMetadata>().unwrap();
}

#[test]
fn can_access_default_content_vector() {
    let dc: DefaultContent = Access::DefaultContent.access().unwrap();
    assert!(dc.iter().any(|s| s == "vi-VN"));
}
