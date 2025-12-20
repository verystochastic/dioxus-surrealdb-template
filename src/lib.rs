// Library exports for testing and reusability

pub mod db;
pub mod server_functions;

// Re-export commonly used types
pub use db::Idea;

#[cfg(feature = "server")]
pub use db::IdeaRecord;
