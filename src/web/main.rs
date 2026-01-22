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
    issues: Vec<Issue>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Issue {
    id: u32,
    title: String,
    state: String,
    assignee: String,
}

async fn list() -> Result<Html<String>, ApiError> {
    let result = git_issue::list(None, None, None);

    let result = match result {
        Ok(result) => result,
        Err(_) => {
            return Err(ApiError::InternalServerError);
        }
    };

    for info in result.infos {
        println!("{}", info);
    }

    let mut issues_collection = Vec::new();

    for issue in &result.value.issues {
        issues_collection.push(Issue {
            id: issue.id,
            title: issue.data["title"].clone(),
            state: issue.data["state"].clone(),
            assignee: issue.data["assignee"].clone(),
        });
    }

    let html = ListTemplate { issues: issues_collection }.render().unwrap();

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
