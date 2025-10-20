use axum::Router;

use crate::state::AppState;

pub mod project;

pub fn app_router() -> Router<AppState> {
    Router::new().merge(project::router())
}
