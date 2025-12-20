use serde::{Deserialize, Serialize};

/// Idea model for storing user-submitted ideas (shared between client and server)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Idea {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    #[serde(default)]
    pub what_must_be_true: Vec<String>,
    #[serde(default)]
    pub development_notes: String,
}

// Server-side internal representation with SurrealDB types
#[cfg(feature = "server")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeaRecord {
    pub id: Option<surrealdb::sql::Thing>,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    #[serde(default)]
    pub what_must_be_true: Vec<String>,
    #[serde(default)]
    pub development_notes: String,
}

#[cfg(feature = "server")]
impl From<IdeaRecord> for Idea {
    fn from(record: IdeaRecord) -> Self {
        Idea {
            id: record.id.map(|thing| thing.to_string()),
            title: record.title,
            description: record.description,
            tags: record.tags,
            what_must_be_true: record.what_must_be_true,
            development_notes: record.development_notes,
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

    /// Get a test database instance (uses memory-based storage for tests)
    /// This is used in integration tests to avoid polluting the production database
    pub async fn get_test_db() -> &'static Surreal<surrealdb::engine::local::Db> {
        use surrealdb::engine::local::Mem;

        static TEST_DB: OnceCell<Surreal<surrealdb::engine::local::Db>> = OnceCell::const_new();

        TEST_DB.get_or_init(|| async {
            let db = Surreal::new::<Mem>(())
                .await
                .expect("Failed to create test database");

            db.use_ns("test_ns")
                .use_db("test_db")
                .await
                .expect("Failed to select test namespace and database");

            db
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Unit tests for the Idea data model
    /// These tests demonstrate TDD for data structures and serialization

    #[test]
    fn test_idea_creation() {
        let idea = Idea {
            id: None,
            title: "Test Idea".to_string(),
            description: "A test description".to_string(),
            tags: vec!["test".to_string(), "rust".to_string()],
            what_must_be_true: vec![],
            development_notes: String::new(),
        };

        assert_eq!(idea.title, "Test Idea");
        assert_eq!(idea.tags.len(), 2);
        assert!(idea.id.is_none());
    }

    #[test]
    fn test_idea_with_development_fields() {
        let idea = Idea {
            id: Some("ideas:test123".to_string()),
            title: "Developed Idea".to_string(),
            description: "Has development fields".to_string(),
            tags: vec![],
            what_must_be_true: vec![
                "Must be tested".to_string(),
                "Must be documented".to_string(),
            ],
            development_notes: "These are my notes".to_string(),
        };

        assert_eq!(idea.what_must_be_true.len(), 2);
        assert_eq!(idea.development_notes, "These are my notes");
        assert!(idea.id.is_some());
    }

    #[test]
    fn test_idea_serialization() {
        let idea = Idea {
            id: Some("ideas:abc123".to_string()),
            title: "Serializable".to_string(),
            description: "Can be serialized to JSON".to_string(),
            tags: vec!["json".to_string()],
            what_must_be_true: vec!["Must serialize".to_string()],
            development_notes: "Test notes".to_string(),
        };

        // Test serialization
        let json = serde_json::to_string(&idea).expect("Failed to serialize");
        assert!(json.contains("Serializable"));
        assert!(json.contains("Must serialize"));

        // Test deserialization
        let deserialized: Idea = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized.title, idea.title);
        assert_eq!(deserialized.what_must_be_true, idea.what_must_be_true);
    }

    #[test]
    fn test_idea_default_fields() {
        // Test that serde(default) works for development fields
        let json = r#"{
            "id": "ideas:test",
            "title": "Old Idea",
            "description": "From before development fields existed",
            "tags": ["old"]
        }"#;

        let idea: Idea = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(idea.what_must_be_true.len(), 0);
        assert_eq!(idea.development_notes, "");
    }

    #[test]
    fn test_idea_equality() {
        let idea1 = Idea {
            id: Some("ideas:1".to_string()),
            title: "Same".to_string(),
            description: "Same".to_string(),
            tags: vec![],
            what_must_be_true: vec![],
            development_notes: String::new(),
        };

        let idea2 = Idea {
            id: Some("ideas:1".to_string()),
            title: "Same".to_string(),
            description: "Same".to_string(),
            tags: vec![],
            what_must_be_true: vec![],
            development_notes: String::new(),
        };

        assert_eq!(idea1, idea2);
    }

    #[cfg(feature = "server")]
    mod server_tests {
        use super::super::*;

        #[test]
        fn test_idea_record_creation() {
            let record = IdeaRecord {
                id: None,
                title: "Test Record".to_string(),
                description: "Server-side record".to_string(),
                tags: vec!["server".to_string()],
                what_must_be_true: vec!["Must work on server".to_string()],
                development_notes: "Server notes".to_string(),
            };

            assert_eq!(record.title, "Test Record");
            assert!(record.id.is_none());
        }

        #[test]
        fn test_idea_from_record_conversion() {
            use surrealdb::sql::Thing;

            let record = IdeaRecord {
                id: Some(Thing::from(("ideas", "test123"))),
                title: "Convert Me".to_string(),
                description: "Test conversion".to_string(),
                tags: vec!["convert".to_string()],
                what_must_be_true: vec!["Must convert".to_string()],
                development_notes: "Conversion notes".to_string(),
            };

            let idea: Idea = record.into();
            assert_eq!(idea.title, "Convert Me");
            assert!(idea.id.is_some());
            assert!(idea.id.unwrap().contains("ideas:test123"));
            assert_eq!(idea.what_must_be_true[0], "Must convert");
        }
    }
}
