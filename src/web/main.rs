use askama::Template;
use axum::{Json, Router, extract::Path, response::Html, response::IntoResponse, routing::get};
use serde_json::{self, Value, json};

enum ApiError {
    NotFound,
    BadRequest(String),
    InternalServerError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ApiError::NotFound => (axum::http::StatusCode::NOT_FOUND, "Resource not found".to_string()),
            ApiError::BadRequest(msg) => (axum::http::StatusCode::BAD_REQUEST, format!("Bad Request: {}", msg)),
            ApiError::InternalServerError => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

async fn ping() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "message": "Service is running",
    }))
}

#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate {
    columns: Vec<String>,
    ids: Vec<u32>,
    rows: Vec<Vec<String>>,
}

async fn list() -> Result<Html<String>, ApiError> {
    let columns = vec!["id".to_string(), "title".to_string(), "state".to_string(), "assignee".to_string()];
    let result = git_issue::list(Some(columns), None, None);

    let result = match result {
        Ok(result) => result,
        Err(_) => {
            return Err(ApiError::InternalServerError);
        }
    };

    for info in result.infos {
        println!("{}", info);
    }

    let columns = result.value.columns;

    let mut ids: Vec<u32> = Vec::new();
    let mut rows: Vec<Vec<String>> = Vec::new();

    for issue in &result.value.issues {
        ids.push(issue.id);

        let mut issue_rows: Vec<String> = Vec::new();

        for col in &columns {
            issue_rows.push(issue.data.get(col).cloned().unwrap_or_default());
        }

        rows.push(issue_rows);
    }

    let issue_collection = ListTemplate { columns, ids, rows };

    let html = issue_collection.render().unwrap();

    Ok(Html(html))
}

async fn show(Path(id): Path<u32>) -> Result<Json<Value>, ApiError> {
    if id == 0 {
        return Err(ApiError::BadRequest("ID does not exist".to_string()));
    }

    let data = vec!["Not implemented yet"];
    Ok(Json(json!({ "id": id,"items": data })))
}

async fn not_found() -> impl IntoResponse {
    ApiError::NotFound
}

fn create_app() -> Router {
    Router::new()
        .route("/ping", get(ping))
        .route("/", get(list))
        .route("/show/{id}", get(show))
        .fallback(not_found)
}

#[tokio::main]
async fn main() {
    let app = create_app();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878")
        .await
        .expect("Failed to bind listener");

    axum::serve(listener, app).await.expect("Failed to start server");
}
