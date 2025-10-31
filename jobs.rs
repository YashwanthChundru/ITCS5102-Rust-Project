use axum::{routing::{get, post}, Router, Json};
use serde::{Deserialize, Serialize};
use crate::db::Db;

#[derive(Deserialize)]
struct CreateJob { title: String, company: String, location: String, description: String }

#[derive(Serialize)]
struct JobOut { id: String, title: String, company: String, location: String }

pub fn routes(_pool: Db) -> Router {
    Router::new()
        .route("/", get(list_jobs).post(create_job))
}

async fn list_jobs() -> Json<Vec<JobOut>> {
    Json(vec![
        JobOut { id: "1".into(), title: "Backend Engineer".into(), company: "Acme".into(), location: "Remote".into() }
    ])
}

async fn create_job(Json(j): Json<CreateJob>) -> Json<JobOut> {
    Json(JobOut { id: "new".into(), title: j.title, company: j.company, location: j.location })
}
