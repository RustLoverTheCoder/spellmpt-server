use anyhow::{Ok, Result};
use chrono::Utc;
use entity::block;
use entity::block::Entity as Block;
use migration::{
    sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set},
    DbErr,
};
use reels_config::contants::DB;
use uuid::Uuid;

pub async fn create_post_by_parent_id(
    parent_id: Option<Uuid>,
    user_id: Uuid,
) -> Result<block::Model, DbErr> {
    let db = DB.get().unwrap();
    let status = 1; //正常
    let create_at = Utc::now();
    let new_block = block::ActiveModel {
        id: Set(Uuid::new_v4().to_owned()),
        parent_id: Set(parent_id.to_owned()),
        user_id: Set(user_id.to_owned()),
        status: Set(status.to_owned()),
        created_at: Set(create_at.to_owned().into()),
        ..Default::default()
    };
    let result = new_block.insert(db).await;
    return result;
}
