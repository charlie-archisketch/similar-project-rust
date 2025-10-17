use mongodb::{Collection, Database, bson::doc, options::UpdateOptions};

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
}
