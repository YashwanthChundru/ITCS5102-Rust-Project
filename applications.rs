use axum::{routing::{get, post}, Router, Json};
use serde::{Deserialize, Serialize};
use crate::db::Db;

#[derive(Deserialize)]
struct ApplyReq { job_id: String, resume_text: String }

#[derive(Serialize)]
struct ApplicationOut { id: String, job_id: String, ats_score: Option<f32> }

pub fn routes(_pool: Db) -> Router {
    Router::new()
        .route("/", post(apply).get(list))
}

async fn apply(Json(req): Json<ApplyReq>) -> Json<ApplicationOut> {
    Json(ApplicationOut { id: "app_1".into(), job_id: req.job_id, ats_score: None })
}

async fn list() -> Json<Vec<ApplicationOut>> {
    Json(vec![])
}
