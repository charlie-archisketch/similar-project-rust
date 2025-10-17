use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transformation {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub z: Option<f64>,
}

impl Transformation {
    pub fn merge_from(&mut self, other: Option<&Self>) {
        if let Some(other) = other {
            if other.x.is_some() {
                self.x = other.x;
            }
            if other.y.is_some() {
                self.y = other.y;
            }
            if other.z.is_some() {
                self.z = other.z;
            }
        }
    }
}
