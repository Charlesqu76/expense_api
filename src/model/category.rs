use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub icon: String,
    // pub create_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct NewCategory {
    pub name: String,
    pub icon: String,
}
