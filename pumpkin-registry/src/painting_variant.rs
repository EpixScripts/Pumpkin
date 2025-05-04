use pumpkin_util::{resource_location::ResourceLocation, text::TextComponent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaintingVariant {
    width: i32,
    height: i32,
    asset_id: ResourceLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<TextComponent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<TextComponent>,
}
