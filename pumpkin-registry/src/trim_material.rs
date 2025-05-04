use pumpkin_util::text::TextComponent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrimMaterial {
    // #[serde(flatten)]
    // assets: MaterialAssetGroup
    asset_name: String, // TODO: replace with above
    description: TextComponent,
}
