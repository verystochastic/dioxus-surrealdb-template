use serde::{Deserialize, Serialize};

/// Idea model for storing user-submitted ideas (shared between client and server)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Idea {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
}

// Server-side internal representation with SurrealDB types
#[cfg(feature = "server")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeaRecord {
    pub id: Option<surrealdb::sql::Thing>,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
}

#[cfg(feature = "server")]
impl From<IdeaRecord> for Idea {
    fn from(record: IdeaRecord) -> Self {
        Idea {
            id: record.id.map(|thing| thing.to_string()),
            title: record.title,
            description: record.description,
            tags: record.tags,
        }
    }
}

// Server-only database code
#[cfg(feature = "server")]
pub mod server {
    use super::*;
    use surrealdb::{engine::local::RocksDb, Surreal};
    use tokio::sync::OnceCell;

    /// Static database instance that's lazily initialized
    static DB: OnceCell<Surreal<surrealdb::engine::local::Db>> = OnceCell::const_new();

    /// Get or initialize the database instance
    pub async fn get_db() -> &'static Surreal<surrealdb::engine::local::Db> {
        DB.get_or_init(|| async {
            // Use RocksDB-based local database
            let db = Surreal::new::<RocksDb>("ideas.db")
                .await
                .expect("Failed to create database");

            db.use_ns("ideas_ns")
                .use_db("ideas_db")
                .await
                .expect("Failed to select namespace and database");

            db
        })
        .await
    }
}
