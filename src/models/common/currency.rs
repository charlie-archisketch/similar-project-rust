use serde::{Deserialize, Serialize};

const CURRENCY_CODES: &[&str] = &["KRW", "USD", "EUR", "JPY", "GBP", "CNY", "VND", "TWD"];

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Default)]
#[allow(clippy::upper_case_acronyms)]
pub enum CurrencyCode {
    #[default]
    #[serde(rename = "KRW")]
    KRW,
    #[serde(rename = "USD")]
    USD,
    #[serde(rename = "EUR")]
    EUR,
    #[serde(rename = "JPY")]
    JPY,
    #[serde(rename = "GBP")]
    GBP,
    #[serde(rename = "CNY")]
    CNY,
    #[serde(rename = "VND")]
    VND,
    #[serde(rename = "TWD")]
    TWD,
}

impl CurrencyCode {
    pub const fn desc(&self) -> &'static str {
        match self {
            CurrencyCode::KRW => "대한민국 원",
            CurrencyCode::USD => "미국 달러",
            CurrencyCode::EUR => "유로",
            CurrencyCode::JPY => "일본 엔",
            CurrencyCode::GBP => "영국 파운드",
            CurrencyCode::CNY => "중국 위안",
            CurrencyCode::VND => "베트남 동",
            CurrencyCode::TWD => "신대만 달러",
        }
    }

    pub const fn sign(&self) -> &'static str {
        match self {
            CurrencyCode::KRW => "₩",
            CurrencyCode::USD => "$",
            CurrencyCode::EUR => "€",
            CurrencyCode::JPY => "¥",
            CurrencyCode::GBP => "£",
            CurrencyCode::CNY => "¥",
            CurrencyCode::VND => "₫",
            CurrencyCode::TWD => "NT$",
        }
    }

    pub const fn default_currency() -> Self {
        CurrencyCode::KRW
    }

    pub fn contains(value: &str) -> bool {
        let upper = value.to_uppercase();
        CURRENCY_CODES.iter().any(|code| upper.contains(code))
    }

    pub fn extract_currency(value: &str) -> Option<&'static str> {
        let upper = value.to_uppercase();
        CURRENCY_CODES
            .iter()
            .find(|code| upper.contains(**code))
            .copied()
    }

    pub fn exclude_currency(value: &str) -> String {
        let mut result = value.to_string();
        for code in CURRENCY_CODES {
            result = result.replace(code, "");
        }
        result
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_uppercase().as_str() {
            "KRW" => Some(CurrencyCode::KRW),
            "USD" => Some(CurrencyCode::USD),
            "EUR" => Some(CurrencyCode::EUR),
            "JPY" => Some(CurrencyCode::JPY),
            "GBP" => Some(CurrencyCode::GBP),
            "CNY" => Some(CurrencyCode::CNY),
            "VND" => Some(CurrencyCode::VND),
            "TWD" => Some(CurrencyCode::TWD),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            CurrencyCode::KRW => "KRW",
            CurrencyCode::USD => "USD",
            CurrencyCode::EUR => "EUR",
            CurrencyCode::JPY => "JPY",
            CurrencyCode::GBP => "GBP",
            CurrencyCode::CNY => "CNY",
            CurrencyCode::VND => "VND",
            CurrencyCode::TWD => "TWD",
        }
    }
}
