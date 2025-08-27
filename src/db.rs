use sqlx::PgPool;
use std::env;

pub async fn connect_db() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env or Railway config");
    PgPool::connect(&database_url).await
}
