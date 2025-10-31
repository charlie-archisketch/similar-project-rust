use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::project_handler::{
        create_project_structure, create_recent_project_structures, get_project_by_id,
        get_project_renderings, get_room_items, get_similar_floors, get_similar_rooms,
    },
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/projects/{project_id}", get(get_project_by_id))
        .route(
            "/projects/{project_id}/renderings",
            get(get_project_renderings),
        )
        .route("/projects/{room_key}/room-items", get(get_room_items))
        .route(
            "/projects/{floor_id}/similar-floor",
            get(get_similar_floors),
        )
        .route("/projects/{room_id}/similar-room", get(get_similar_rooms))
        .route(
            "/projects/{project_id}/structure",
            post(create_project_structure),
        )
        .route(
            "/projects/structures",
            post(create_recent_project_structures),
        )
}
