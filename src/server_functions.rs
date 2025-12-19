use crate::db::Idea;
use dioxus::prelude::*;

/// Submit a new idea to the database
#[post("/api/ideas/submit")]
pub async fn submit_idea_server(
    title: String,
    description: String,
    tags: Vec<String>,
) -> Result<Idea> {
    #[cfg(feature = "server")]
    {
        use crate::db::{server::get_db, IdeaRecord};

        let idea = IdeaRecord {
            id: None,
            title,
            description,
            tags,
        };

        // Insert into SurrealDB
        let db = get_db().await;
        let created: Option<IdeaRecord> = db
            .create("ideas")
            .content(idea)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        let created = created.ok_or_else(|| ServerFnError::new("Failed to create idea"))?;
        Ok(created.into())
    }

    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new("Server-only function"))
    }
}

/// Get all ideas from the database
#[post("/api/ideas/all")]
pub async fn get_all_ideas_server() -> Result<Vec<Idea>> {
    #[cfg(feature = "server")]
    {
        use crate::db::{server::get_db, IdeaRecord};

        let db = get_db().await;
        let ideas: Vec<IdeaRecord> = db
            .select("ideas")
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(ideas.into_iter().map(|record| record.into()).collect())
    }

    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new("Server-only function"))
    }
}

/// Delete an idea from the database by ID
#[post("/api/ideas/delete")]
pub async fn delete_idea_server(id: String) -> Result<()> {
    #[cfg(feature = "server")]
    {
        use crate::db::server::get_db;

        let db = get_db().await;

        // SurrealDB delete by full record ID string (format: "ideas:xyz")
        let _deleted: Vec<crate::db::IdeaRecord> = db
            .delete(id.as_str())
            .await
            .map_err(|e| ServerFnError::new(format!("Delete failed for ID {}: {}", id, e)))?;

        Ok(())
    }

    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new("Server-only function"))
    }
}
