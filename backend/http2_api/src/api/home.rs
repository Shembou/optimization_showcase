use axum::response::IntoResponse;

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Plain text greeting", body = String)
    )
)]
pub async fn hello() -> impl IntoResponse {
    "Hello world!"
}