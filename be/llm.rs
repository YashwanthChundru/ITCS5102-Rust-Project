use anyhow::Result;
use std::collections::HashSet;

// Placeholder: replace with Ollama calls
pub async fn llm_score_resume(resume: &str, jd: &str) -> Result<f32> {
    let r: HashSet<_> = resume.to_lowercase().split_whitespace().collect();
    let j: HashSet<_> = jd.to_lowercase().split_whitespace().collect();
    let inter = r.intersection(&j).count() as f32;
    let union = r.union(&j).count() as f32;
    Ok(if union > 0.0 { (inter / union) * 100.0 } else { 0.0 })
}
