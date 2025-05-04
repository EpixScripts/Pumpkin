use pumpkin_util::text::TextComponent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JukeboxSong {
    sound_event: String, // TODO: SoundEvent
    description: TextComponent,
    length_in_seconds: f32, // TODO: enforce PositiveFloat
    comparator_output: i32, // TODO: enforce range
}
