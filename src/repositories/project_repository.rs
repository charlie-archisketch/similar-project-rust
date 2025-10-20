use mongodb::{Collection, Database, bson::doc, options::FindOptions, options::UpdateOptions};

use crate::{error::ApiError, models::project::Project};

#[derive(Clone)]
pub struct ProjectRepository {
    collection: Collection<Project>,
}

impl ProjectRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("projects"),
        }
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Project, ApiError> {
        let project = self
            .collection
            .find_one(doc! { "_id": id }, None)
            .await
            .map_err(ApiError::internal)?
            .ok_or_else(|| ApiError::not_found(format!("Project {id} not found")))?;

        Ok(project)
    }

    pub async fn persist_default_cover_image(
        &self,
        project_id: &str,
        url: &str,
    ) -> Result<(), ApiError> {
        self.collection
            .update_one(
                doc! { "_id": project_id },
                doc! { "$set": { "defaultCoverImage": url } },
                UpdateOptions::builder().upsert(false).build(),
            )
            .await
            .map_err(ApiError::internal)?;
        Ok(())
    }

    pub async fn find_recent_ids(&self, limit: i64) -> Result<Vec<String>, ApiError> {
        if limit <= 0 {
            return Ok(Vec::new());
        }

        let options = FindOptions::builder()
            .sort(doc! { "updatedAt": -1 })
            .limit(limit)
            .build();

        let mut cursor = self
            .collection
            .find(doc! {}, options)
            .await
            .map_err(ApiError::internal)?;

        let mut ids = Vec::new();
        while cursor.advance().await.map_err(ApiError::internal)? {
            let project: Project = cursor.deserialize_current().map_err(ApiError::internal)?;

            if let Some(id) = project.id {
                ids.push(id);
            }
        }

        Ok(ids)
    }
}
