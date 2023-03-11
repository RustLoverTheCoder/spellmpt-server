use anyhow::{Ok, Result};
use entity::user;
use entity::user::Entity as User;
use migration::sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use reels_config::contants::DB;

pub async fn find_user_by_phone(phone: String) -> Result<Option<user::Model>> {
    let db = DB.get().unwrap();
    let user = User::find()
        .filter(user::Column::PhoneNumber.eq(phone))
        .one(db)
        .await
        .expect("Cannot retrieve user");
    Ok(user)
}

pub async fn create_user() -> Result<()> {
    Ok(())
}

pub async fn get_user_info() -> Result<()> {
    Ok(())
}

pub async fn update_user_info() -> Result<()> {
    Ok(())
}
