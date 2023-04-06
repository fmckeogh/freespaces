use {
    crate::{error::Error, STATIC_FILES_MAX_AGE},
    axum::{
        extract::{Path, State},
        http::header::{HeaderMap, HeaderValue, CACHE_CONTROL, CONTENT_TYPE},
        response::IntoResponse,
    },
    sqlx::{Pool, Postgres},
};

/// Fetches
pub async fn images(
    State(db): State<Pool<Postgres>>,
    Path(image_filename): Path<String>,
) -> Result<impl IntoResponse, Error> {
    let contents = sqlx::query!(
        "SELECT contents FROM images WHERE filename = $1",
        image_filename
    )
    .fetch_one(&db)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => Error::FileNotFound(image_filename.clone()),
        _ => Error::DatabaseError(e),
    })?
    .contents;

    let mime_type = mime_guess::from_path(image_filename).first_or_text_plain();

    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str(mime_type.as_ref()).unwrap(),
    );
    headers.insert(
        CACHE_CONTROL,
        HeaderValue::from_str(&format!(
            "public, max-age={STATIC_FILES_MAX_AGE}, immutable"
        ))
        .unwrap(),
    );

    Ok((headers, contents))
}
