use axum::{extract::State, http::StatusCode, Json};
use scrdesk_shared::error::Result;
use std::sync::Arc;

use crate::AppState;

pub async fn list_users(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<Vec<scrdesk_shared::models::user::UserResponse>>)> {
    let users = sqlx::query_as::<_, scrdesk_shared::models::user::User>(
        "SELECT * FROM users LIMIT 100"
    )
    .fetch_all(&state.db_pool)
    .await?;

    let user_responses: Vec<_> = users.into_iter().map(|u| u.into()).collect();

    Ok((StatusCode::OK, Json(user_responses)))
}
