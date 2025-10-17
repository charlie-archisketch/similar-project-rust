use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::common::CurrencyCode;
use crate::models::project::enums::ShowFloorplan;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Portfolio {
    pub id: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub order: String,
    #[serde(default)]
    pub image_ids: Vec<String>,
    #[serde(default)]
    pub show_floorplan: ShowFloorplan,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

impl Portfolio {
    pub fn new(
        id: String,
        title: String,
        description: Option<String>,
        order: String,
        image_ids: Vec<String>,
        show_floorplan: ShowFloorplan,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            title,
            description,
            order,
            image_ids,
            show_floorplan,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub value: f64,
    pub unit: CurrencyCode,
}

impl Price {
    pub fn from(price: &str) -> Option<Self> {
        let trimmed = price.trim();
        if trimmed.is_empty() {
            return None;
        }

        if !CurrencyCode::contains(trimmed) {
            return None;
        }

        let currency_code = CurrencyCode::extract_currency(trimmed)?;
        let value_part = CurrencyCode::exclude_currency(trimmed);
        let value_part = value_part.trim();
        if value_part.is_empty() {
            return None;
        }

        if !value_part
            .chars()
            .all(|ch| ch.is_ascii_digit() || ch == '.' || ch.is_ascii_whitespace())
        {
            return None;
        }

        let numeric_str = value_part.split_whitespace().next()?;
        let value = numeric_str.parse::<f64>().ok()?;
        let unit = CurrencyCode::parse(currency_code)?;

        Some(Self { value, unit })
    }

    pub fn of(value: f64, unit: CurrencyCode) -> Self {
        Self { value, unit }
    }
}
