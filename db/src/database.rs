use sqlx::postgres::PgPool;
use model::error::DatabaseError;

#[derive(Debug)]
pub struct Database {
	pub pool: PgPool
}

impl Database {
	pub async fn new(database_url: &str) -> Result<Database, DatabaseError> {
		let pool = PgPool::builder()
			.max_size(5)
			.build(database_url)
			.await?;
		
		let db = Database { pool };
		Ok(db)
	}
}