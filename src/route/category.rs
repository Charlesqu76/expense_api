// src/routes/expenses.rs
use crate::models::{Category, NewCategory};
use actix_web::{get, post, web, HttpResponse, Responder};
use log::error;
use sqlx::PgPool;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_category).service(create_category);
}

#[get("category")]
async fn get_category(pool: web::Data<PgPool>) -> impl Responder {
    let results = sqlx::query_as::<_, Category>("SELECT * FROM expense.category")
        .fetch_all(pool.get_ref())
        .await;

    match results {
        Ok(expenses) => HttpResponse::Ok().json(expenses),
        Err(err) => {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("category")]
async fn create_category(
    pool: web::Data<PgPool>,
    category: web::Json<NewCategory>,
) -> impl Responder {
    let result = sqlx::query_as::<_, Category>(
        "INSERT INTO expense.category (name, category) 
         VALUES ($1, $2) 
         RETURNING *",
    )
    .bind(&category.name)
    .bind(&category.category)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(expense) => HttpResponse::Created().json(expense),
        Err(err) => {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
