use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::models::common::Transformation;

use crate::models::project::child::floorplan::{Floorplan, Room};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundingBox {
    pub width: f64,
    pub height: f64,
    pub area: f64,
    pub aspect: f64,
    pub aspect_ri: f64,
}

impl BoundingBox {
    pub fn from_floorplan(floorplan: &Floorplan) -> Result<Self> {
        let inner_points: Vec<&Transformation> = floorplan
            .rooms
            .iter()
            .flat_map(|room| room.inner_points.iter())
            .collect();

        if inner_points.is_empty() {
            return Err(anyhow!("Inner points must not be empty"));
        }

        compute_bounding_box(&inner_points)
    }

    pub fn from_room(floorplan: &Floorplan, room: &Room) -> Result<Self> {
        if !floorplan.rooms.contains(room) {
            return Err(anyhow!("Room is not part of the provided floorplan"));
        }

        if room.inner_points.is_empty() {
            return Err(anyhow!("Inner points must not be empty"));
        }

        let inner_points: Vec<&Transformation> = room.inner_points.iter().collect();
        compute_bounding_box(&inner_points)
    }
}

fn compute_bounding_box(points: &[&Transformation]) -> Result<BoundingBox> {
    let xs: Vec<f64> = points
        .iter()
        .filter_map(|p| p.x)
        .collect();
    let zs: Vec<f64> = points
        .iter()
        .filter_map(|p| p.z)
        .collect();

    if xs.is_empty() || zs.is_empty() {
        return Err(anyhow!("Unable to compute bounding box without coordinates"));
    }

    let min_x = xs
        .iter()
        .cloned()
        .fold(f64::INFINITY, |acc, value| acc.min(value));
    let max_x = xs
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, |acc, value| acc.max(value));

    let min_z = zs
        .iter()
        .cloned()
        .fold(f64::INFINITY, |acc, value| acc.min(value));
    let max_z = zs
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, |acc, value| acc.max(value));

    let width = (max_x - min_x) * 2.0;
    let height = (max_z - min_z) * 2.0;
    let area = width * height;

    let aspect = if height != 0.0 {
        width / height
    } else {
        f64::INFINITY
    };

    let aspect_ri = if width != 0.0 && height != 0.0 {
        width.max(height) / width.min(height)
    } else {
        f64::INFINITY
    };

    Ok(BoundingBox {
        width,
        height,
        area,
        aspect,
        aspect_ri,
    })
}
