use axum::{Router, routing::get};

use crate::{handlers::project_handler::get_project_by_id, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/projects/{project_id}", get(get_project_by_id))
}
