use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreData {
    pub score: u32,
    pub date: String,
}
#[derive(Serialize, Deserialize)]
pub struct ScoreList {
    scores: Vec<ScoreData>,
}

const SCORE_FILE: &str = "scores.json";

pub fn load_scores() -> Result<Vec<ScoreData>, Box<dyn std::error::Error>> {
    if !Path::new(SCORE_FILE).exists() {
        println!("Score file not found");
        return Ok(Vec::new());
    }

    let file_content = fs::read_to_string(SCORE_FILE)?;
    let score_list: ScoreList = serde_json::from_str(&file_content)?;
    Ok(score_list.scores)
}

pub fn save_score(score: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut scores = load_scores().unwrap_or_default();

    let new_score = ScoreData {
        score,
        date: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };

    scores.push(new_score);
    scores.sort_by(|a, b| b.score.cmp(&a.score));
    scores.truncate(10); // Keep only top 10 scores

    let json = serde_json::to_string_pretty(&scores)?;
    fs::write(SCORE_FILE, json)?;

    Ok(())
}