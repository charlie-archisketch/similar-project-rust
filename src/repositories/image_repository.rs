use mongodb::{
    Collection, Database,
    bson::{Bson, doc},
    options::FindOptions,
};

use crate::{error::ApiError, models::image::Image};

#[derive(Clone)]
pub struct ImageRepository {
    collection: Collection<Image>,
}

impl ImageRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("images"),
        }
    }

    pub async fn find_by_ids(&self, ids: &[String]) -> Result<Vec<Image>, ApiError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let bson_ids: Vec<Bson> = ids.iter().cloned().map(Bson::String).collect();
        let filter = doc! { "_id": { "$in": bson_ids } };
        let options = FindOptions::builder()
            .projection(doc! {
                "_id": 1,
                "type": 1,
                "status": 1,
                "resolution": 1,
            })
            .build();

        let mut cursor = self
            .collection
            .find(filter, options)
            .await
            .map_err(ApiError::internal)?;

        let mut images = Vec::new();
        while cursor.advance().await.map_err(ApiError::internal)? {
            let image: Image = cursor.deserialize_current().map_err(ApiError::internal)?;
            images.push(image);
        }

        Ok(images)
    }
}
