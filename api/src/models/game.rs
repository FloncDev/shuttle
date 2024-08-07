// Holds models reprisenting parts of a badminton match.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, types::Json, FromRow, Row};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub rating: f64,
    pub uncertainty: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub id: i32,
    pub player_one: Player,
    pub player_two: Player,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Set(i32, i32);

#[derive(Debug, Serialize, Deserialize)]
pub struct DuoGame {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub first_to: i32,
    pub team_one: Team,
    pub team_two: Team,
    pub sets: Json<Vec<[i32; 2]>>,
}

impl<'r> FromRow<'r, PgRow> for DuoGame {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let id = row.try_get("id")?;
        let created_at = row.try_get("created_at")?;
        let first_to = row.try_get("first_to")?;
        let sets = row.try_get("sets")?;

        let t1p1 = Player {
            id: row.try_get("t1p1_id")?,
            name: row.try_get("t1p1_name")?,
            rating: row.try_get("t1p1_rating")?,
            uncertainty: row.try_get("t1p1_uncertainty")?,
        };

        let t1p2 = Player {
            id: row.try_get("t1p2_id")?,
            name: row.try_get("t1p2_name")?,
            rating: row.try_get("t1p2_rating")?,
            uncertainty: row.try_get("t1p2_uncertainty")?,
        };

        let t2p1 = Player {
            id: row.try_get("t2p1_id")?,
            name: row.try_get("t2p1_name")?,
            rating: row.try_get("t2p1_rating")?,
            uncertainty: row.try_get("t2p1_uncertainty")?,
        };

        let t2p2 = Player {
            id: row.try_get("t2p2_id")?,
            name: row.try_get("t2p2_name")?,
            rating: row.try_get("t2p2_rating")?,
            uncertainty: row.try_get("t2p2_uncertainty")?,
        };

        let team_one = Team {
            id: row.try_get("t1_id")?,
            player_one: t1p1,
            player_two: t1p2,
        };

        let team_two = Team {
            id: row.try_get("t2_id")?,
            player_one: t2p1,
            player_two: t2p2,
        };

        Ok(DuoGame {
            id,
            created_at,
            first_to,
            team_one,
            team_two,
            sets,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoloGame {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub first_to: i32,
    pub player_one: Player,
    pub player_two: Player,
    pub sets: Json<Vec<[i32; 2]>>,
}

impl<'r> FromRow<'r, PgRow> for SoloGame {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let id = row.try_get("id")?;
        let created_at = row.try_get("created_at")?;
        let first_to = row.try_get("first_to")?;
        let sets = row.try_get("sets")?;

        let player_one = Player {
            id: row.try_get("p1_id")?,
            name: row.try_get("p1_name")?,
            rating: row.try_get("p1_rating")?,
            uncertainty: row.try_get("p1_uncertainty")?,
        };

        let player_two = Player {
            id: row.try_get("p2_id")?,
            name: row.try_get("p2_name")?,
            rating: row.try_get("p2_rating")?,
            uncertainty: row.try_get("p2_uncertainty")?,
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
