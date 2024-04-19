use sqlx::PgPool;
use crate::init::DB_POOL;

pub mod user;

pub  fn fetch_db_pool() -> &'static PgPool{
    &*DB_POOL
}

