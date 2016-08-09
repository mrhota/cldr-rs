#![cfg(test)]

use core::*;

#[test]
fn reading_data_works() {
    let item: AvailableLocales = Access::AvailableLocales.access().unwrap();
    assert!(item.modern.iter().any(|s| s == "zh-Hans"));
}

#[test]
fn reading_unicode_works() {
    let item1: ScriptMetadata = Access::ScriptMetadata("Hans".to_owned()).access().unwrap();
    let item2: ScriptMetadata = Access::ScriptMetadata("Hebr".to_owned()).access().unwrap();
    assert_eq!(item1.sample_char, '字');
    assert_eq!(item2.sample_char, 'א');
}

#[test]
#[should_panic]
fn reading_nonexistent_script_fails() {
    let item: ScriptMetadata = Access::ScriptMetadata("Hurdur".to_owned()).access().unwrap();
}
