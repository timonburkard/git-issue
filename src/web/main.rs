use askama::Template;
use axum::extract::Query;
use axum::http::header;
use axum::{Json, Router, extract::Path, response::Html, response::IntoResponse, routing::get};
use serde::Deserialize;
use serde_json::{self, json};
use std::fs;
use std::str::FromStr;

use git_issue::model::{Filter, load_settings};

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

struct Data {
    key: String,
    value: String,
}

#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate {
    ids: Vec<u32>,
    rows: Vec<Vec<Data>>,
    user: String,
    columns: Vec<String>,
    filters: Vec<String>,
}

#[derive(Template)]
#[template(path = "show.html")]
struct ShowTemplate {
    id: u32,
    content: String,
}

#[derive(Deserialize)]
struct ListColumnsQuery {
    #[serde(default, deserialize_with = "comma_separated")]
    columns: Vec<String>,
}

#[derive(Deserialize)]
struct ListFiltersQuery {
    #[serde(default, deserialize_with = "comma_separated")]
    filters: Vec<String>,
}

fn comma_separated<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    Ok(s.unwrap_or_default().split(',').map(|s| s.to_string()).collect())
}

async fn list(Query(columns): Query<ListColumnsQuery>, Query(filters): Query<ListFiltersQuery>) -> Result<Html<String>, ApiError> {
    let columns = if columns.columns.is_empty() { None } else { Some(columns.columns) };

    let mut filters_parsed: Vec<Filter> = Vec::new();

    for filter in &filters.filters {
        // Skip empty filter strings
        if filter.trim().is_empty() {
            continue;
        }

        let parsed = Filter::from_str(&filter);
        match parsed {
            Ok(f) => filters_parsed.push(f),
            Err(_) => {
                return Err(ApiError::BadRequest(format!("Invalid filter format: {}", filter)));
            }
        }
    }

    let result = git_issue::list(columns, Some(filters_parsed), None);

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
    let mut rows: Vec<Vec<Data>> = Vec::new();

    for issue in &result.value.issues {
        ids.push(issue.id);

        let mut issue_rows: Vec<Data> = Vec::new();

        for col in &columns {
            issue_rows.push(Data {
                key: col.clone(),
                value: issue.data.get(col).cloned().unwrap_or_default(),
            });
        }

        rows.push(issue_rows);
    }

    let (settings, _) = match load_settings() {
        Ok(settings) => settings,
        Err(_) => {
            return Err(ApiError::InternalServerError);
        }
    };

    let issue_collection = ListTemplate {
        ids,
        rows,
        user: settings.user.to_string(),
        columns,
        filters: filters.filters,
    };

    let html = issue_collection.render().unwrap();

    Ok(Html(html))
}

async fn show(Path(id): Path<u32>) -> Result<Html<String>, ApiError> {
    if id == 0 {
        return Err(ApiError::BadRequest("ID does not exist".to_string()));
    }

    let result = git_issue::show(id);

    let md_path = match result {
        Ok(result) => result.value,
        Err(_) => {
            return Err(ApiError::InternalServerError);
        }
    };

    let content = match fs::read_to_string(&md_path) {
        Ok(content) => content,
        Err(_) => {
            return Err(ApiError::InternalServerError);
        }
    };

    let template = ShowTemplate { id, content };
    let html = template.render().unwrap();

    Ok(Html(html))
}

async fn favicon() -> impl IntoResponse {
    let bytes = include_bytes!("favicon.ico");
    ([(header::CONTENT_TYPE, "image/x-icon")], bytes.as_slice()).into_response()
}

async fn not_found() -> impl IntoResponse {
    ApiError::NotFound
}

fn create_app() -> Router {
    Router::new()
        .route("/ping", get(ping))
        .route("/", get(list))
        .route("/list", get(list))
        .route("/show/{id}", get(show))
        .route("/favicon.ico", get(favicon))
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
