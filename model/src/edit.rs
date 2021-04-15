use uuid::Uuid;
use sqlx::PgPool;
use crate::error::DatabaseError;
use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::Type)]
#[sqlx(rename="action")]
pub enum Action {
    #[sqlx(rename="CREATE")]
    Create,
    #[sqlx(rename="DELETE")]
    Delete,
    #[sqlx(rename="EDIT")]
    Edit
}

#[derive(Debug, sqlx::FromRow)]
pub struct Edit {
    pub id: Uuid,
    pub project: Uuid,
    pub action_type: Action,
    pub create_at: DateTime<Utc>,
    pub author_id: Uuid
}

impl Edit {
    pub async fn create(pool: &PgPool, project_id: Uuid, action: Action, author_id: Uuid) -> Result<Edit, DatabaseError> {
        let result = sqlx::query_as!(
            Edit,
            r#"
                INSERT INTO edits (project, action_type, author_id) VALUES ($1, $2, $3) RETURNING id, project, create_at, author_id, action_type AS "action_type!: Action";
            "#,
            project_id,
            action as Action,
            author_id
        )
            .fetch_one(pool)
            .await;

        match result {
            Ok(edit) => {
                Ok(edit)
            }
            Err(err) => {
                eprintln!("{}", err);
                Err(err.into())
            }
        }
    }

    pub async fn get_for_project(pool: &PgPool, project_id: Uuid) -> Result<Vec<Edit>, DatabaseError> {
        let edits_row = sqlx::query_as!(
            Edit,
            r#"
                SELECT id, project, create_at, author_id, action_type AS "action_type!: Action" FROM edits WHERE project = $1;
            "#,
            project_id
        )
            .fetch_all(pool)
            .await?;

        Ok(edits_row)
    }

    pub async fn get_last_for_project(pool: &PgPool, project_id: Uuid) -> Result<Edit, DatabaseError> {
        let edits_row = sqlx::query_as!(
            Edit,
            r#"
                SELECT id, project, create_at, author_id, action_type AS "action_type!: Action" FROM edits WHERE project = $1 ORDER BY create_at DESC LIMIT 1;
            "#,
            project_id
        )
            .fetch_one(pool)
            .await?;

        Ok(edits_row)
    }
}
