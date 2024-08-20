use crate::models::{Expense, NewExpense};
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
