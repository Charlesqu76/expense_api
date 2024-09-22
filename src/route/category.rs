use crate::model::category::{Category, NewCategory};
use actix_web::{get, post, web, HttpResponse, Responder};
use log::error;
use sqlx::PgPool;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_category)
        .service(create_category)
        .service(edit_category);
}

#[get("category")]
async fn get_category(pool: web::Data<PgPool>) -> impl Responder {
    let results = sqlx::query_as::<_, Category>("SELECT * FROM expense.category ORDER BY id")
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
    println!("{:?}", category);
    let result = sqlx::query_as::<_, Category>(
        "INSERT INTO expense.category (name, icon) 
         VALUES ($1, $2) 
         RETURNING *",
    )
    .bind(&category.name)
    .bind(&category.icon)
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

#[post("category/edit")]
async fn edit_category(pool: web::Data<PgPool>, category: web::Json<Category>) -> impl Responder {
    println!("{:?}", category);
    let result = sqlx::query_as::<_, Category>(
        "UPDATE expense.category
         SET name =$1, icon = $2 
         WHERE id = ($3)
         RETURNING *",
    )
    .bind(&category.name)
    .bind(&category.icon)
    .bind(&category.id)
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
