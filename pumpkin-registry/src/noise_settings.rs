use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NoiseSettings {
    min_y: i32,
    height: i32,
    #[serde(rename = "size_horizontal")]
    noise_size_horizontal: i32,
    #[serde(rename = "size_vertical")]
    noise_size_vertical: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NoiseGeneratorSettings {
    #[serde(rename = "noise")]
    noise_settings: NoiseSettings,
    // default_block: BlockState,
    // default_fluid: BlockState,
    // noise_router: NoiseRouter,
    // surface_rule: SurfaceRule
    // spawn_target: ParameterPoint,
    sea_level: i32,
    disable_mob_generation: bool,
    aquifers_enabled: bool,
    ore_veins_enabled: bool,
    #[serde(rename = "legacy_random_source")]
    use_legacy_random_source: bool,
}