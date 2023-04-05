use anyhow::{Ok, Result};
use chrono::Utc;
use config::contants::DB;
use entity::user;
use entity::user::Entity as User;
use migration::{
    sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set},
    DbErr,
};
use uuid::Uuid;

pub async fn find_user_by_phone(phone: String) -> Result<Option<user::Model>> {
    let db = DB.get().unwrap();
    let user = User::find()
        .filter(user::Column::PhoneNumber.eq(phone))
        .one(db)
        .await?;
    Ok(user)
}

pub async fn create_user(phone: String) -> Result<user::Model, DbErr> {
    let db = DB.get().unwrap();
    let status = 0;
    let create_at = Utc::now();
    let new_user = user::ActiveModel {
        id: Set(Uuid::new_v4().to_owned()),
        nickname: Set(phone.to_owned()),
        phone_number: Set(phone.to_owned()),
        status: Set(status.to_owned()),
        created_at: Set(create_at.to_owned().into()),
        ..Default::default() // all other attributes are `NotSet`
    };
    let result = new_user.insert(db).await;
    return result;
}

pub async fn get_user_info_by_id(user_id: Uuid) -> Result<Option<user::Model>, DbErr> {
    let db = DB.get().unwrap();
    let user = User::find_by_id(user_id).one(db).await;
    user
}

pub async fn update_user_info_by_id(
    user_id: Uuid,
    nickname: Option<String>,
    phone_number: Option<String>,
) -> Result<user::Model, DbErr> {
    let db = DB.get().unwrap();
    let mut update_user = user::ActiveModel {
        id: Set(user_id.to_owned()),
        ..Default::default() // all other attributes are `NotSet`
    };
    if let Some(nickname) = nickname {
        update_user.nickname = Set(nickname.to_owned());
    }
    if let Some(phone_number) = phone_number {
        update_user.phone_number = Set(phone_number.to_owned());
    }
    let result = update_user.update(db).await;
    return result;
}
