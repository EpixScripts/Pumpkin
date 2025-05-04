use pumpkin_protocol::codec::var_int::VarInt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum TemperatureModifier {
    None,
    Frozen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ClimateSettings {
    has_precipitation: bool,
    temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature_modifier: Option<TemperatureModifier>,
    downfall: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
enum GrassColorModifier {
    #[default]
    None,
    DarkForest,
    Swamp,
}

// TODO: use `minecraft:particle_type` registry
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ParticleOptions {
    r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<VarInt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AmbientParticleSettings {
    options: ParticleOptions,
    probability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AmbientMoodSettings {
    #[serde(rename = "sound")]
    sound_event: String, // TODO: SoundEvent,
    tick_delay: i32,
    block_search_extent: i32,
    #[serde(rename = "offset")]
    sound_position_offset: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AmbientAdditionsSettings {
    #[serde(rename = "sound")]
    sound_event: String, // TODO: SoundEvent
    tick_chance: f64,
}

fn default_background_music_volume() -> f32 {
    1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BiomeSpecialEffects {
    fog_color: i32,
    water_color: i32,
    water_fog_color: i32,
    sky_color: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    foliage_color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dry_foliage_color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grass_color: Option<i32>,
    #[serde(default)]
    grass_color_modifier: GrassColorModifier,
    #[serde(rename = "particle", skip_serializing_if = "Option::is_none")]
    ambient_particle_settings: Option<AmbientParticleSettings>,
    #[serde(rename = "ambient_sound", skip_serializing_if = "Option::is_none")]
    ambient_loop_sound_event: Option<String>, // TODO: Option<SoundEvent>
    #[serde(rename = "mood_sound", skip_serializing_if = "Option::is_none")]
    ambient_mood_settings: Option<AmbientMoodSettings>,
    #[serde(rename = "additions_sound", skip_serializing_if = "Option::is_none")]
    ambient_additions_settings: Option<AmbientAdditionsSettings>,
    // TODO
    // #[serde(rename = "music", skip_serializing_if = "Option::is_none")]
    // background_music: WeightedList<Music>,
    #[serde(rename = "music_volume", default = "default_background_music_volume")]
    background_music_volume: f32,
}

// TODO
#[derive(Debug, Clone, Serialize, Deserialize)]
struct BiomeGenerationSettings {
    // carvers: ,
    // features: ,
}

// TODO
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MobSpawnSettings {
    // #[serde(rename = "creature_spawn_probability")]
    // creature_generation_probability: ,
    // spawners: ,
    // #[serde(rename = "spawn_costs")]
    // mob_spawn_costs: ,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Biome {
    #[serde(flatten)]
    climate_settings: ClimateSettings,
    #[serde(rename = "effects")]
    special_effects: BiomeSpecialEffects,
    #[serde(flatten)]
    generation_settings: BiomeGenerationSettings,
    #[serde(flatten)]
    mob_settings: MobSpawnSettings,
}
