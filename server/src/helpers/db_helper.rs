use sea_orm::{Database, DatabaseConnection};

use super::dev_helper::get_dev;

pub async fn db_connection() -> DatabaseConnection {
    let db_url =get_dev("DATABASE_URL".to_string(), Some("sqlite://database.db?mode=rwc".to_owned())).await;
    let db = Database::connect(db_url).await.unwrap();
    db
}
