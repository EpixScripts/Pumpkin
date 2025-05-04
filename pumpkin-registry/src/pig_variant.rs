use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PigVariant {
    asset_id: String, // TODO: ModelAndTexture
    // TODO: spawn_conditions
}
