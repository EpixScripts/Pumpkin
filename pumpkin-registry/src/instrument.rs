use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    sound_event: String, // TODO: SoundEvent
    use_duration: f32, // TODO: enforce PositiveFloat
    range: f32, // TODO: enforce PositiveFloat
    // TODO: is TextComponent ready?
    //  description: TextComponent<'static>,
}
