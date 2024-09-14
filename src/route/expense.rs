use crate::models::{Expense, NewExpense, UpdateExpense};
use actix_web::{get, post, web, HttpResponse, Responder};
use log::error;
use sqlx::PgPool;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_expenses).service(create_expense);
}

#[get("expense")]
async fn get_expenses(pool: web::Data<PgPool>) -> impl Responder {
    let results = sqlx::query_as::<_, Expense>("SELECT * FROM expense.expense")
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

#[post("expense")]
async fn create_expense(
    pool: web::Data<PgPool>,
    new_expense: web::Json<NewExpense>,
) -> impl Responder {
    let result = sqlx::query_as::<_, Expense>(
        "INSERT INTO expense.expense (amount, date, description, category_id ) 
         VALUES ($1, $2, $3, $4) 
         RETURNING *",
    )
    .bind(new_expense.amount)
    .bind(new_expense.date)
    .bind(&new_expense.description)
    .bind(&new_expense.category_id)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(expense) => HttpResponse::Created().json(expense),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("update")]
async fn update_expense(
    pool: web::Data<PgPool>,
    new_expense: web::Json<UpdateExpense>,
) -> impl Responder {
    let result = sqlx::query_as::<_, Expense>(
        "UPDATE expense.expense SET amount = $1, date = $2, description = $3, category_id = $4 WHERE id = $5",
    )
    .bind(new_expense.amount)
    .bind(new_expense.date)
    .bind(&new_expense.description)
    .bind(&new_expense.category_id)
    .bind(&new_expense.id)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(expense) => HttpResponse::Ok().json(expense),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("delete")]
async fn delete_expense(pool: web::Data<PgPool>, id: web::Json<i32>) -> impl Responder {
    let result = sqlx::query("DELETE FROM expense.expense WHERE id = $1")
        .bind(id.0)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(id.0),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
