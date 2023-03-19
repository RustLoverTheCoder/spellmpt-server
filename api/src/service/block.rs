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

pub async fn find_block_by_id(id: Uuid) -> Result<Option<block::Model>, DbErr> {
    let db = DB.get().unwrap();
    let block = Block::find_by_id(id).one(db).await;
    block
}

pub async fn update_block_by_modal(
    block: block::Model,
    parent_id: Option<Uuid>,
    title: Option<String>,
    status: Option<i32>,
) -> Result<block::Model, DbErr> {
    let update_at = Utc::now();
    let db = DB.get().unwrap();
    let mut block: block::ActiveModel = block.into();
    block.title = Set(title.to_owned());
    block.parent_id = Set(parent_id.to_owned());
    match status {
        Some(status) => block.status = Set(status.to_owned()),
        None => {}
    };
    block.updated_at = Set(Some(update_at.to_owned().into()));
    let block_result = block.update(db).await;
    block_result
}


pub async fn find_all_blocks(user_id: Uuid, parent_id: Option<Uuid>) -> Result<Vec<block::Model>, DbErr> {
    let db = DB.get().unwrap();
    let mut query = Block::find().filter(block::Column::UserId.eq(user_id));
    if let Some(parent_id) = parent_id {
        query = query.filter(block::Column::ParentId.eq(parent_id));
    }
    let blocks = query.all(db).await;
    blocks
}
