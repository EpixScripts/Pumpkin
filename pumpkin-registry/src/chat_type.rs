use pumpkin_util::text::style::Style;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatType {
    pub chat: ChatTypeDecoration,
    pub narration: ChatTypeDecoration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatTypeDecoration {
    pub translation_key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
    pub parameters: Vec<String>, // TODO: Vec<Parameter>
}
