// Holds models reprisenting parts of a badminton match.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub id: i32,
    pub player_one: User,
    pub player_two: Option<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Set {
    pub id: i32,
    pub team_one_score: i32,
    pub team_two_score: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub first_to: i32,
    pub team_one: Team,
    pub team_two: Team,
    pub sets: Vec<Set>,
}
