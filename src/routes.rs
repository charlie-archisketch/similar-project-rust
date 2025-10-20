use axum::Router;

use crate::state::AppState;

pub mod project_router;

pub fn app_router() -> Router<AppState> {
    Router::new().merge(project_router::router())
}
