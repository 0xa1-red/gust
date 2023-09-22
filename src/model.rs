use std::time::SystemTime;
use serde::Serialize;

use diesel::prelude::*;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name=crate::schema::gusts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Gust {
    pub key: String,
    pub content: Vec<u8>,
    pub expires: Option<SystemTime>,
    pub anonymous: Option<bool>,
    pub is_volatile: Option<bool>,
    pub reads_remaining: Option<i32>,
    pub visibility: String,
    pub created_at: SystemTime,
    pub accessed: Option<i32>,
    pub starred: Option<i32>,
    pub title: String,
}

#[derive(Serialize, Debug)]
pub struct GustDisplay {
    pub key: String,
    pub title: String,
    pub content: String,
    pub created_at: SystemTime,
    pub accessed: Option<i32>,
    pub starred: Option<i32>,
}