use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DamageScaling {
    Never,
    WhenCausedByLivingNonPlayer,
    Always,
}

// TODO: associated SoundEvents
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DamageEffects {
    Hurt,
    Thorns,
    Drowning,
    Burning,
    Poking,
    Freezing
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DeathMessageType {
    #[default]
    Default,
    FallVariants,
    IntentionalGameDesign,
}

fn default_damage_type_effects() -> DamageEffects {
    DamageEffects::Hurt
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageType {
    message_id: String,
    scaling: DamageScaling,
    exhaustion: f32,
    #[serde(default = "default_damage_type_effects")]
    effects: DamageEffects,
    #[serde(default)]
    death_message_type: DeathMessageType,
}
