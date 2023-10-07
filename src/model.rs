use std::{
    ops::Add,
    time::SystemTime,
};
use serde::{Serialize, Deserialize};
use parse_duration::parse;

use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, Serialize)]
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
    pub language: String,
}

impl From<NewGust> for Gust {
    fn from(value: NewGust) -> Self {
        let mut expiry: Option<SystemTime> = None;
        match value.expires {
            Some(dur) => {
                let parsed_duration = parse(&dur);
                match parsed_duration {
                    Err(_) => {},
                    Ok(std_duration) => {
                        expiry = Some(SystemTime::now().add(std_duration));
                    }
                }
            },
            None => {},
        }

        Gust{
            key: String::from(""),
            content: value.content.into_bytes(),
            expires: expiry,
            anonymous: value.anonymous.or(Some(false)),
            is_volatile: value.is_volatile.or(Some(false)),
            reads_remaining: value.reads_remaining.or(Some(1)),
            visibility: value.visibility.unwrap_or(String::from("public")),
            created_at: std::time::SystemTime::now(),
            accessed: Some(0),
            starred: Some(0),
            title: value.title,
            language: value.language.unwrap_or(String::from("text")),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct GustDisplay {
    pub key: String,
    pub title: String,
    pub content: String,
    pub created_at: SystemTime,
    pub expires_at: Option<SystemTime>,
    pub accessed: i32,
    pub starred: i32,
    pub language: String,
}

impl From<Gust> for GustDisplay {
    fn from(value: Gust) -> Self {
        GustDisplay {
            key: value.key,
            title: value.title,
            content: String::from_utf8(value.content).expect("Found invalid UTF-8"),
            created_at: value.created_at,
            expires_at: value.expires,
            accessed: value.accessed.unwrap_or(0),
            starred: value.starred.unwrap_or(0),
            language: value.language,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct GustListItem {
    pub key: String,
    pub title: String,
    pub created_at: SystemTime,
    pub accessed: Option<i32>,
    pub starred: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct NewGust {
    pub content: String,
    pub expires: Option<String>,
    pub anonymous: Option<bool>,
    pub is_volatile: Option<bool>,
    pub reads_remaining: Option<i32>,
    pub visibility: Option<String>,
    pub title: String,
    pub language: Option<String>,
}