use {
    crate::STATIC_FILES_MAX_AGE,
    axum::{
        http::{
            header::{CACHE_CONTROL, CONTENT_TYPE},
            HeaderMap, HeaderValue, StatusCode, Uri,
        },
        response::{IntoResponse, Response},
    },
    include_dir::{include_dir, Dir},
};

static DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/frontend");

/// Handler for static files
pub async fn static_files(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');

    let Some(file) = DIR.get_file(path) else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let mime_type = mime_guess::from_path(path).first_or_text_plain();

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

    (headers, file.contents()).into_response()
}
