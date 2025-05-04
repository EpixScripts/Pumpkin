use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FrogVariant {
    asset_id: String, // TODO: ClientAsset
    // TODO: spawn_conditions
}
