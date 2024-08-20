// src/models.rs
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Expense {
    pub id: i32,
    pub amount: f64,
    pub date: NaiveDate,
    pub category_id: Option<i32>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct NewExpense {
    pub amount: f64,
    pub date: NaiveDate,
    pub category_id: Option<i32>,
    pub description: Option<String>,
}

// Add similar structures for Category and PaymentMethod
