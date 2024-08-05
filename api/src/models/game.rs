// Holds models reprisenting parts of a badminton match.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, types::Json, FromRow, Row};

use crate::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub id: i32,
    pub player_one: User,
    pub player_two: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Set(i32, i32);

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DuoGame {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub first_to: i32,
    pub team_one: Team,
    pub team_two: Team,
    pub sets: Vec<[i32; 2]>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoloGame {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub first_to: i32,
    pub player_one: User,
    pub player_two: User,
    pub sets: Json<Vec<[i32; 2]>>,
}

impl<'r> FromRow<'r, PgRow> for SoloGame {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let id = row.try_get("id")?;
        let created_at = row.try_get("created_at")?;
        let first_to = row.try_get("first_to")?;
        let sets = row.try_get("sets")?;

        let player_one = User {
            id: row.try_get("p1_id")?,
            name: row.try_get("p1_name")?,
            created_at: row.try_get("p1_created_at")?,
        };

        let player_two = User {
            id: row.try_get("p2_id")?,
            name: row.try_get("p2_name")?,
            created_at: row.try_get("p2_created_at")?,
        };

        Ok(SoloGame {
            id,
            created_at,
            first_to,
            player_one,
            player_two,
            sets,
        })
    }
}
