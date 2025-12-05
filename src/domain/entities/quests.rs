use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::infrastructure::postgres::schema::quests;


#[derive(Debug, Clone,Queryable,Identifiable,Selectable)]
#[diesel(table_name = quests)]
pub struct QuestEntity {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub guild_commander_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}


#[derive(Debug, Clone,Queryable,Insertable)]
#[diesel(table_name = quests)]
pub struct AddQuestEntity {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub guild_commander_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone,Queryable,AsChangeset)]
#[diesel(table_name = quests)]
pub struct EditQuestEntity {
    pub name: Option<String>,
    pub description: Option<String>,
    pub guild_commander_id: i32,
    pub updated_at: NaiveDateTime,
}
