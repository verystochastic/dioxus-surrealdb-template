/// Integration tests for database operations
/// Run with: cargo test --test db_tests --features server
///
/// These tests demonstrate TDD for database-backed features:
/// 1. Write a failing test first
/// 2. Implement the minimal code to make it pass
/// 3. Refactor while keeping tests green

#[cfg(feature = "server")]
mod db_integration {
    use dioxus_surrealdb_template::db::{server::get_test_db, Idea, IdeaRecord};
    use surrealdb::Surreal;

    /// Helper function to set up a fresh test database
    async fn setup_test_db() -> &'static Surreal<surrealdb::engine::local::Db> {
        get_test_db().await
    }

    /// Helper to create a test idea
    fn create_test_idea(title: &str, description: &str) -> IdeaRecord {
        IdeaRecord {
            id: None,
            title: title.to_string(),
            description: description.to_string(),
            tags: vec!["test".to_string()],
            what_must_be_true: vec![],
            development_notes: String::new(),
        }
    }

    #[tokio::test]
    async fn test_create_idea() {
        let db = setup_test_db().await;

        let idea = create_test_idea("Test Idea", "This is a test");

        // Create the idea
        let created: Option<IdeaRecord> = db
            .create("ideas")
            .content(idea.clone())
            .await
            .expect("Failed to create idea");

        assert!(created.is_some());
        let created = created.unwrap();
        assert_eq!(created.title, "Test Idea");
        assert_eq!(created.description, "This is a test");
        assert!(created.id.is_some());
    }

    #[tokio::test]
    async fn test_get_all_ideas() {
        let db = setup_test_db().await;

        // Create multiple test ideas
        let idea1 = create_test_idea("Idea 1", "First test idea");
        let idea2 = create_test_idea("Idea 2", "Second test idea");

        let _: Option<IdeaRecord> = db.create("ideas").content(idea1).await.unwrap();
        let _: Option<IdeaRecord> = db.create("ideas").content(idea2).await.unwrap();

        // Get all ideas
        let ideas: Vec<IdeaRecord> = db.select("ideas").await.expect("Failed to get ideas");

        assert!(ideas.len() >= 2);
        assert!(ideas.iter().any(|i| i.title == "Idea 1"));
        assert!(ideas.iter().any(|i| i.title == "Idea 2"));
    }

    #[tokio::test]
    async fn test_update_idea_development_fields() {
        let db = setup_test_db().await;

        // Create an idea
        let idea = create_test_idea("Update Test", "Testing updates");
        let created: Option<IdeaRecord> = db.create("ideas").content(idea).await.unwrap();
        let created = created.unwrap();
        let id = created.id.unwrap();

        // Update with development fields
        let updated = IdeaRecord {
            id: None,
            title: "Update Test".to_string(),
            description: "Testing updates".to_string(),
            tags: vec!["test".to_string()],
            what_must_be_true: vec![
                "Must have tests".to_string(),
                "Must be fast".to_string(),
            ],
            development_notes: "This is a note".to_string(),
        };

        let result: Option<IdeaRecord> = db
            .update((id.tb.as_str(), id.id.to_string().as_str()))
            .content(updated)
            .await
            .expect("Failed to update");

        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.what_must_be_true.len(), 2);
        assert_eq!(result.development_notes, "This is a note");
    }

    #[tokio::test]
    async fn test_delete_idea() {
        let db = setup_test_db().await;

        // Create an idea
        let idea = create_test_idea("Delete Test", "Will be deleted");
        let created: Option<IdeaRecord> = db.create("ideas").content(idea).await.unwrap();
        let created = created.unwrap();
        let id = created.id.unwrap();

        // Delete it
        let deleted: Option<IdeaRecord> = db
            .delete((id.tb.as_str(), id.id.to_string().as_str()))
            .await
            .expect("Failed to delete");

        assert!(deleted.is_some());

        // Verify it's gone
        let result: Option<IdeaRecord> = db
            .select((id.tb.as_str(), id.id.to_string().as_str()))
            .await
            .expect("Failed to select");

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_idea_with_empty_development_fields() {
        let db = setup_test_db().await;

        let idea = IdeaRecord {
            id: None,
            title: "Empty Fields".to_string(),
            description: "Testing empty development fields".to_string(),
            tags: vec![],
            what_must_be_true: vec![],
            development_notes: String::new(),
        };

        let created: Option<IdeaRecord> = db.create("ideas").content(idea).await.unwrap();

        assert!(created.is_some());
        let created = created.unwrap();
        assert_eq!(created.what_must_be_true.len(), 0);
        assert_eq!(created.development_notes, "");
    }
}
