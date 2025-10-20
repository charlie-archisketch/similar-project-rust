use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ShowFloorplan {
    #[default]
    None,
    TwoD,
    ThreeD,
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
