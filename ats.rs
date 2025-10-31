use axum::{routing::post, Router, Json};
use serde::Deserialize;
use crate::{db::Db, services::llm_score_resume};

#[derive(Deserialize)]
struct ScoreReq { resume_text: String, job_description: String }

pub fn routes(_pool: Db) -> Router {
    Router::new().route("/score", post(score))
}

async fn score(Json(req): Json<ScoreReq>) -> Json<serde_json::Value> {
    let score = llm_score_resume(&req.resume_text, &req.job_description).await.unwrap_or(0.0);
    Json(serde_json::json!({ "score": score }))
}
