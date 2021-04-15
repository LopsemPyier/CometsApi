use sqlx::postgres::{ PgPool, PgPoolOptions };
use model::error::DatabaseError;

#[derive(Debug)]
pub struct Database {
	pub pool: PgPool
}

impl Database {
	pub async fn new(database_url: &str) -> Result<Database, DatabaseError> {
		let pool = PgPoolOptions::new()
			.max_connections(5)
			.connect(database_url)
			.await?;
		
		let db = Database { pool };
		Ok(db)
	}
}