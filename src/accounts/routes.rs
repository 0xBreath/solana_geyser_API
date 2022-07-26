use crate::accounts::{Account, Accounts};
use crate::error_handler::CustomError;
use actix_web::web::{Json, Path};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/")]
async fn root() -> Result<HttpResponse, CustomError> {
    let message = String::from("Welcome to root!");
    Ok(HttpResponse::Ok().json(json!({ "message": message })))
}

#[get("/accounts")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let accounts = Accounts::find_all()?;
    Ok(HttpResponse::Ok().json(accounts))
}

#[get("/accounts/{id}")]
async fn find_one(id: Path<i32>) -> Result<HttpResponse, CustomError> {
    let account = Accounts::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(account))
}

#[post("/accounts")]
async fn create(account: Json<Account>) -> Result<HttpResponse, CustomError> {
    let account = Accounts::create(account.into_inner())?;
    Ok(HttpResponse::Ok().json(account))
}

#[put("/accounts/{id}")]
async fn update(id: Path<i32>, account: Json<Account>) -> Result<HttpResponse, CustomError> {
    let account = Accounts::update(id.into_inner(), account.into_inner())?;
    Ok(HttpResponse::Ok().json(account))
}

#[delete("/accounts/{id}")]
async fn delete(id: Path<i32>) -> Result<HttpResponse, CustomError> {
    let account = Accounts::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": account })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(root);
    config.service(find_all);
    config.service(find_one);
    config.service(create);
    config.service(update);
    config.service(delete);
}
