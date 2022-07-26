use crate::db;
use crate::error_handler::CustomError;
use crate::schema::accounts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// Insertion and ops on table
#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "accounts"]
pub struct Account {
    pub key: String,
    pub owner: String,
}
// Diesel ORM map to table
#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "accounts"]
pub struct Accounts {
    pub id: i32,
    pub key: String,
    pub owner: String,
}

impl Accounts {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let accounts = accounts::table.load::<Accounts>(&conn)?;
        Ok(accounts)
    }
    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let account = accounts::table.filter(accounts::id.eq(id)).first(&conn)?;
        Ok(account)
    }
    pub fn create(account: Account) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let account = Account::from(account);
        let account = diesel::insert_into(accounts::table)
            .values(account)
            .get_result(&conn)?;
        Ok(account)
    }
    pub fn update(id: i32, account: Account) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let account = diesel::update(accounts::table)
            .filter(accounts::id.eq(id))
            .set(account)
            .get_result(&conn)?;
        Ok(account)
    }
    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(accounts::table.filter(accounts::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl Account {
    fn from(account: Account) -> Account {
        Account {
            key: account.key,
            owner: account.owner,
        }
    }
}
