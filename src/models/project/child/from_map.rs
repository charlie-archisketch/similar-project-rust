use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FromMap {
    pub map_fp_data_id: Option<String>,
    pub address_id: Option<String>,
    pub fp_data_id: Option<String>,
    pub map_floorplan_id: Option<String>,
}

impl FromMap {
    pub fn new(
        map_fp_data_id: Option<String>,
        address_id: Option<String>,
        fp_data_id: Option<String>,
        map_floorplan_id: Option<String>,
    ) -> Self {
        Self {
            map_fp_data_id,
            address_id,
            fp_data_id,
            map_floorplan_id,
        }
    }
}
