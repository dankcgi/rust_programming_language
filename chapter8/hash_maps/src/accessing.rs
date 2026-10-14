use std::collections::HashMap;

pub fn get_team_score(team_name: String, scores: HashMap<String, i32>) -> i32 {
  scores.get(&team_name).copied().unwrap_or(0)
}

pub fn iterate_scores(scores: HashMap<String, i32>) {
  for (key, value) in &scores {
    println!("{key}: {value}");
  }
}