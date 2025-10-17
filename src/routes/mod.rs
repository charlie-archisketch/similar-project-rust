use axum::Router;

use crate::state::AppState;

pub mod project_routes;

pub fn app_router() -> Router<AppState> {
    Router::new().merge(project_routes::router())
}
