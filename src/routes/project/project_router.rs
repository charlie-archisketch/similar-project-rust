use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::project_handler::{
        create_project_structure, create_recent_project_structures, get_project_by_id,
    },
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/projects/{project_id}", get(get_project_by_id))
        .route(
            "/projects/{project_id}/structure",
            post(create_project_structure),
        )
        .route(
            "/projects/structures",
            post(create_recent_project_structures),
        )
}
