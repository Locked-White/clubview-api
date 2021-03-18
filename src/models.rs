use crate::schema::*;
use serde::{Deserialize, Serialize};
use diesel_derive_enum::DbEnum;
use serde_json;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, PartialEq, DbEnum)]
pub enum MessageType {
    Normal,
    Audio,
    Video,
    File,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[table_name = "messages"]
pub struct Massage {
    pub id: uuid::Uuid,
    pub author_id: String,
    pub content_type: MessageType,
    pub message: String,
    pub channel: uuid::Uuid,
    pub meta: serde_json::Value,
    pub posted_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[table_name = "channels"]
pub struct Channel {
    pub id: uuid::Uuid,
    pub messages: Vec<uuid::Uuid>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, PartialEq, DbEnum)]
pub enum ClubInfoComponentType {
    Text, //<p>
    Header, // <h(1,2,3,4,5)>
    Image, // <img>
    Video,
    Slider,
    Container, // <div>
    Chat,
    Sns,
    News,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[table_name = "club_info_components"]
pub struct ClubInfoComponent {
    pub id: uuid::Uuid,
    pub info_type: ClubInfoComponentType,
    pub css: String,
    pub content: serde_json::Value,
}

#[derive(Debug, PartialEq, DbEnum)]
pub enum ClubCategory {
    Sports,
    Culture,
    Creative,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[table_name = "clubs"]
pub struct Club {
    pub id: uuid::Uuid,
    pub name: String,
    pub thumbnail: String,
    pub summary: String,
    pub category: ClubCategory,
    pub article_components: Vec<uuid::Uuid>,
    pub member_count: i32,
    pub inviting: bool,
    pub keywords: Vec<String>,
    pub posted_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}