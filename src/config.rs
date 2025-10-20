use anyhow::Result;

pub struct AppConfig {
    pub port: u16,
    pub mongodb_uri: Option<String>,
    pub mongodb_db: String,
    pub aws_region: Option<String>,
    pub s3_bucket_name: Option<String>,
    pub cdn_url: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let port = std::env::var("PORT")
            .ok()
            .and_then(|port| port.parse::<u16>().ok())
            .unwrap_or(8080);
        let mongodb_uri = std::env::var("MONGODB_URI").ok();
        let mongodb_db =
            std::env::var("MONGODB_DB").unwrap_or_else(|_| "ArchisketchDB".to_string());
        let aws_region = std::env::var("AWS_REGION").ok();
        let s3_bucket_name = std::env::var("S3_BUCKET_NAME").ok();
        let cdn_url = std::env::var("CDN_URL")
            .unwrap_or_else(|_| "https://dev-resources.archisketch.com".to_string());

        Ok(Self {
            port,
            mongodb_uri,
            mongodb_db,
            aws_region,
            s3_bucket_name,
            cdn_url,
        })
    }
}
