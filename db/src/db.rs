use sqlx::postgres::PgPool;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct Database {
	pub pool: PgPool
}

impl Database {
	pub async fn new(database_url: &str) -> Result<Database, Error> {
		let pool = PgPool::builder()
			.max_size(5)
			.build(database_url)
			.await?;
		
		let db = Database { pool };
		Ok(db)
	}
}