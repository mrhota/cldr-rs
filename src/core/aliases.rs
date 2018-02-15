use std::collections::BTreeMap;

#[derive(Deserialize)]
pub enum AliasReason {
    #[serde(rename = "deprecated")]
    Deprecated,
    #[serde(rename = "overlong")]
    OverLong,
    #[serde(rename = "macrolanguage")]
    MacroLanguage,
    #[serde(rename = "legacy")]
    Legacy,
    #[serde(rename = "bibliographic")]
    Bibliographic,
}

impl Default for AliasReason {
    fn default() -> Self {
        AliasReason::Deprecated
    }
}

#[derive(Deserialize, Default)]
pub struct Alias {
    #[serde(rename = "_reason")]
    pub reason: AliasReason,
    #[serde(rename = "_replacement")]
    pub replacement: String,
}

pub type LanguageAlias = BTreeMap<String, Alias>;
pub type ScriptAlias = BTreeMap<String, Alias>;
pub type TerritoryAlias = BTreeMap<String, Alias>;
pub type VariantAlias = BTreeMap<String, Alias>;
pub type ZoneAlias = BTreeMap<String, Alias>;
