use axum::{extract::State, response::Html};

use crate::app_state::AppState;

pub async fn root(State(_state): State<AppState>) -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
