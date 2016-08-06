#[derive(Deserialize)]
pub enum ListGender {
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "mixedNeutral")]
    MixedNeutral,
    #[serde(rename = "maleTaints")]
    MaleTaints
}

impl Default for ListGender {
    fn default() -> Self { ListGender::Neutral }
}
