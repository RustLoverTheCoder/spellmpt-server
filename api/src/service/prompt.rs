use anyhow::{Ok, Result};
use chrono::Utc;
use config::contants::DB;
use entity::prompt;
use entity::prompt::Entity as Prompt;
use migration::{
    sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set},
    DbErr,
};
use uuid::Uuid;

// Define a function to create a new prompt
pub async fn create_prompt_func(
    title: String,
    content: String,
    prompt_type: i32,
    user_id: Uuid,
) -> Result<prompt::Model, DbErr> {
    let db = DB.get().unwrap();
    let status = 0;
    let create_at = Utc::now();
    let new_prompt = prompt::ActiveModel {
        id: Set(Uuid::new_v4().to_owned()),
        title: Set(title.to_owned()),
        content: Set(content.to_owned()),
        prompt_type: Set(prompt_type.to_owned()),
        status: Set(status.to_owned()),
        user_id: Set(user_id.to_owned()),
        created_at: Set(create_at.to_owned().into()),
        ..Default::default() // all other attributes are `NotSet`
    };
    let result = new_prompt.insert(db).await;
    return result;
}

// Define a function to find a prompt by its ID
pub async fn find_prompt_by_id_func(prompt_id: Uuid) -> Result<Option<prompt::Model>, DbErr> {
    let db = DB.get().unwrap();
    let prompt = Prompt::find_by_id(prompt_id).one(db).await;
    prompt
}
