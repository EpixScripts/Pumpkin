use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoiseParameters {
    first_octave: i32,
    amplitudes: Vec<f64>,
}
