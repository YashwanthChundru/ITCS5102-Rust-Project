use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Application {
    pub id: String,
    pub user_id: String,
    pub job_id: String,
    pub resume_text: String,
    pub ats_score: Option<f32>,
    pub created_at: String,
}
