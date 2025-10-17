const PROD_CDN_URL: &str = "https://resources.archisketch.com";
const PROD_I_URL: &str = "https://i.archisketch.com";
const DEV_CDN_URL: &str = "https://dev-resources.archisketch.com";
const DEV_I_URL: &str = "https://dev-i.archisketch.com";

pub fn convert_image_url(image_url: &str, width: u32) -> String {
    let replaced = image_url
        .replace(PROD_CDN_URL, PROD_I_URL)
        .replace(DEV_CDN_URL, DEV_I_URL);
    format!("{replaced}?f=webp&w={width}")
}

#[allow(dead_code)]
pub fn recover_image_url(converted: &str) -> String {
    converted
        .replace(PROD_I_URL, PROD_CDN_URL)
        .replace(DEV_I_URL, DEV_CDN_URL)
        .split('?')
        .next()
        .unwrap_or_default()
        .to_string()
}
