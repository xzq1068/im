use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::db::fetch_db_pool;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, FromRow)]
pub(crate) struct User {
    pub(crate) id: i64,
    pub(crate) account_id: i64,
    pub(crate) credential: String,
    pub(crate) salt: String,
    pub(crate) nickname: String,
    pub(crate) avatar: String,
    pub(crate) signature: String,
    pub(crate) status: UserStatus,
    pub(crate) info: serde_json::Value,
    pub(crate) create_at: DateTime<Utc>,
    pub(crate) update_at: DateTime<Utc>,
    pub(crate) delete_at: DateTime<Utc>,
}

#[derive(
serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Hash
)]
#[sqlx(type_name = "user_status", rename_all = "snake_case")]
pub enum UserStatus {
    NA = 0,
    Online = 1,
    Busy = 2,
    Away = 3,
}


impl User{

    pub async fn get_by_account_id(account_id: i64)->anyhow::Result<Self> {

        let user = sqlx::query_as("SELECT id, account_id, credential, salt, nickname, avatar, signature, status, info, create_at, update_at, delete_at FROM api.user \
        WHERE account_id = $1")
            .bind(account_id)
            .fetch_one(fetch_db_pool())
            .await?;

        Ok(user)
    }

}

#[cfg(test)]
mod tests{
    use std::fmt::Debug;

    use crate::db::user::User;

    #[tokio::test]
    async fn test_get_by_account_id() {
        let user = User::get_by_account_id(1).await.unwrap();

        println!("{:#?}", user);

    }
}