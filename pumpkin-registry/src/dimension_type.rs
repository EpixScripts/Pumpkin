use pumpkin_util::{math::int_provider::IntProvider, resource_location::ResourceLocation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterSettings {
    piglin_safe: bool,
    has_raids: bool,
    monster_spawn_light_level: IntProvider,
    monster_spawn_light_level_limit: i32, // TODO: enforce range
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimension {
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_time: Option<i64>,
    has_skylight: bool,
    has_ceiling: bool,
    ultrawarm: bool,
    natural: bool,
    coordinate_scale: f64, // TODO: enforce range
    bed_works: bool,
    respawn_anchor_works: bool,
    min_y: i32, // TODO: enforce range
    height: i32, // TODO: enforce range
    logical_height: i32, // TODO: enforce range
    infiniburn: String, // TODO: Block codec
    effects: ResourceLocation,
    ambient_light: f32,
    #[serde(flatten)]
    monster_settings: MonsterSettings,
}
