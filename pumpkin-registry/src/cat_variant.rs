use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CatVariant {
    asset_id: String, // TODO: ClientAsset
    // TODO: spawn_conditions
}
