use dotenvy::var;
use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    
    let database_url = match var("DATABASE_URL") {
        Ok(x) => x,
        Err(x)=> panic!("{}", format!("DATABASE_URL env variable not set: {}", x)),
    };
    Database::connect(database_url).await
}


pub async fn init_db() -> Result<DatabaseConnection, DbErr> {
    let db = establish_connection().await?;
    
    
    
    Ok(db)
}
