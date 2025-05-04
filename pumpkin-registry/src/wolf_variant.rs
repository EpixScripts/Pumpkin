use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WolfAssetInfo {
    wild: String, // TODO: ClientAsset
    tame: String, // TODO: ClientAsset
    angry: String, // TODO: ClientAsset
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WolfVariant {
    #[serde(rename = "assets")]
    asset_info: WolfAssetInfo,
    // TODO: spawn_conditions
}
