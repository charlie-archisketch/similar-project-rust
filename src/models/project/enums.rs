use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ShowFloorplan {
    None,
    TwoD,
    ThreeD,
}

impl Default for ShowFloorplan {
    fn default() -> Self {
        ShowFloorplan::None
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ColumnType {
    Cylinder,
    Rectangular,
}

impl ColumnType {
    pub const fn value(&self) -> &'static str {
        match self {
            ColumnType::Cylinder => "cylinder",
            ColumnType::Rectangular => "rectangular",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FinishTargetType {
    Floor,
    Ceiling,
    Wall,
}
