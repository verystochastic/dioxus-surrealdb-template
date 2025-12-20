# Test-Driven Development Guide for Dioxus + Rust

This guide will help you learn Rust and Dioxus through Test-Driven Development (TDD).

## Table of Contents
1. [Testing Setup](#testing-setup)
2. [TDD Workflow](#tdd-workflow)
3. [Types of Tests](#types-of-tests)
4. [Running Tests](#running-tests)
5. [TDD Examples](#tdd-examples)
6. [Best Practices](#best-practices)

## Testing Setup

The project now includes:
- ✅ Test dependencies in `Cargo.toml`
- ✅ Unit tests in `src/db.rs`
- ✅ Integration tests in `tests/db_tests.rs`
- ✅ Test database helper using in-memory SurrealDB

### Dependencies Added:
```toml
[dev-dependencies]
tokio = { version = "1.0", features = ["full", "test-util"] }
tempfile = "3.8"  # For creating temporary test databases
uuid = { version = "1.0", features = ["v4"] }  # For generating test data
```

## TDD Workflow

The classic TDD cycle (Red-Green-Refactor):

### 1. 🔴 Red: Write a Failing Test
Write a test for the feature you want to implement. It should fail because the feature doesn't exist yet.

```rust
#[test]
fn test_new_feature() {
    let result = my_function();
    assert_eq!(result, expected_value);
}
```

### 2. 🟢 Green: Make It Pass
Write the **minimum** code needed to make the test pass. Don't worry about perfect code yet.

```rust
fn my_function() -> SomeType {
    // Minimal implementation
    expected_value
}
```

### 3. 🔵 Refactor: Improve the Code
Now that the test passes, refactor to improve code quality while keeping tests green.

```rust
fn my_function() -> SomeType {
    // Clean, well-structured implementation
    // Tests ensure behavior doesn't change
}
```

## Types of Tests

### 1. Unit Tests (In src/ files)

Test individual functions and data structures in isolation.

**Location**: Same file as the code, in a `#[cfg(test)] mod tests` block

**Example** (src/db.rs:96-237):
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idea_creation() {
        let idea = Idea {
            id: None,
            title: "Test".to_string(),
            description: "Testing".to_string(),
            tags: vec![],
            what_must_be_true: vec![],
            development_notes: String::new(),
        };

        assert_eq!(idea.title, "Test");
    }
}
```

**Run with**:
```bash
cargo test
```

### 2. Integration Tests (In tests/ directory)

Test how different parts of your system work together, including database operations.

**Location**: `tests/*.rs` files

**Example** (tests/db_tests.rs):
```rust
#[tokio::test]
async fn test_create_idea() {
    let db = setup_test_db().await;

    let idea = create_test_idea("Test", "Description");
    let created = db.create("ideas").content(idea).await.unwrap();

    assert!(created.is_some());
}
```

**Run with**:
```bash
cargo test --test db_tests --features server
```

### 3. Server Function Tests

Test your Dioxus server functions (API endpoints).

**Example**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_all_ideas_server() {
        // Setup test data
        // Call server function
        // Assert results
    }
}
```

## Running Tests

### Run All Unit Tests:
```bash
cargo test
```

### Run All Tests (Including Integration):
```bash
cargo test --features server
```

### Run Specific Test:
```bash
cargo test test_idea_creation
```

### Run Tests in a Specific File:
```bash
cargo test --test db_tests
```

### Run Tests with Output:
```bash
cargo test -- --nocapture
```

### Run Tests in Watch Mode (Auto-rerun):
```bash
cargo install cargo-watch
cargo watch -x test
```

## TDD Examples

### Example 1: Adding a New Field (Simple)

**Scenario**: Add a `priority` field to Ideas

#### Step 1: Write the Test (RED)
```rust
#[test]
fn test_idea_has_priority() {
    let idea = Idea {
        // ... existing fields
        priority: Priority::High,
    };

    assert_eq!(idea.priority, Priority::High);
}
```

Run: `cargo test test_idea_has_priority` → ❌ Fails (field doesn't exist)

#### Step 2: Make It Pass (GREEN)
```rust
// In src/db.rs
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}

pub struct Idea {
    // ... existing fields
    #[serde(default = "default_priority")]
    pub priority: Priority,
}

fn default_priority() -> Priority {
    Priority::Medium
}
```

Run: `cargo test test_idea_has_priority` → ✅ Passes

#### Step 3: Refactor (BLUE)
- Add more tests for edge cases
- Consider if Priority should be in its own module
- Add documentation

### Example 2: Adding a Business Logic Function (Medium)

**Scenario**: Add a function to validate idea title length

#### Step 1: Write the Test (RED)
```rust
#[test]
fn test_validate_title_too_short() {
    let result = validate_idea_title("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Title must be at least 3 characters");
}

#[test]
fn test_validate_title_too_long() {
    let long_title = "x".repeat(101);
    let result = validate_idea_title(&long_title);
    assert!(result.is_err());
}

#[test]
fn test_validate_title_valid() {
    let result = validate_idea_title("Good Title");
    assert!(result.is_ok());
}
```

Run: `cargo test validate_title` → ❌ Fails (function doesn't exist)

#### Step 2: Make It Pass (GREEN)
```rust
// In src/db.rs
pub fn validate_idea_title(title: &str) -> Result<(), String> {
    if title.len() < 3 {
        return Err("Title must be at least 3 characters".to_string());
    }
    if title.len() > 100 {
        return Err("Title must be less than 100 characters".to_string());
    }
    Ok(())
}
```

Run: `cargo test validate_title` → ✅ Passes

#### Step 3: Refactor (BLUE)
```rust
const MIN_TITLE_LENGTH: usize = 3;
const MAX_TITLE_LENGTH: usize = 100;

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    TooShort { min: usize },
    TooLong { max: usize },
}

pub fn validate_idea_title(title: &str) -> Result<(), ValidationError> {
    let len = title.len();

    if len < MIN_TITLE_LENGTH {
        return Err(ValidationError::TooShort { min: MIN_TITLE_LENGTH });
    }

    if len > MAX_TITLE_LENGTH {
        return Err(ValidationError::TooLong { max: MAX_TITLE_LENGTH });
    }

    Ok(())
}
```

### Example 3: Database Integration Test (Advanced)

**Scenario**: Test updating idea development notes

#### Step 1: Write the Test (RED)
```rust
// In tests/db_tests.rs
#[tokio::test]
async fn test_update_development_notes() {
    let db = setup_test_db().await;

    // Create an idea
    let idea = create_test_idea("Original", "Desc");
    let created: Option<IdeaRecord> = db
        .create("ideas")
        .content(idea)
        .await
        .unwrap();

    let id = created.unwrap().id.unwrap();

    // Update development notes
    let updated = IdeaRecord {
        id: None,
        title: "Original".to_string(),
        description: "Desc".to_string(),
        tags: vec![],
        what_must_be_true: vec![],
        development_notes: "New notes here".to_string(),
    };

    let result: Option<IdeaRecord> = db
        .update((id.tb.as_str(), id.id.to_string().as_str()))
        .content(updated)
        .await
        .unwrap();

    assert_eq!(result.unwrap().development_notes, "New notes here");
}
```

Run: `cargo test --test db_tests --features server` → Check result

## Best Practices

### 1. Write Tests First
Always write the test before the implementation. This ensures:
- You think about the API design
- You write testable code
- You have clear success criteria

### 2. Keep Tests Simple and Focused
Each test should verify ONE thing:

**Good**:
```rust
#[test]
fn test_title_validation_too_short() {
    let result = validate_title("ab");
    assert!(result.is_err());
}

#[test]
fn test_title_validation_too_long() {
    let result = validate_title(&"x".repeat(101));
    assert!(result.is_err());
}
```

**Bad**:
```rust
#[test]
fn test_title_validation() {
    // Tests multiple things at once
    assert!(validate_title("ab").is_err());
    assert!(validate_title(&"x".repeat(101)).is_err());
    assert!(validate_title("good").is_ok());
}
```

### 3. Use Descriptive Test Names
Test names should describe what they test:

**Good**: `test_validate_title_rejects_empty_string`

**Bad**: `test1`, `test_title`

### 4. Use Test Helpers
Create helper functions to reduce duplication:

```rust
fn create_test_idea(title: &str) -> Idea {
    Idea {
        id: None,
        title: title.to_string(),
        description: "Test description".to_string(),
        tags: vec![],
        what_must_be_true: vec![],
        development_notes: String::new(),
    }
}

#[test]
fn test_something() {
    let idea = create_test_idea("My Idea");
    // ... test logic
}
```

### 5. Test Edge Cases
Always test:
- Empty inputs
- Null/None values
- Maximum values
- Invalid inputs
- Boundary conditions

### 6. Keep Tests Fast
- Use in-memory databases for tests (we use `Mem` instead of `RocksDb`)
- Mock external dependencies
- Parallelize tests (Rust does this by default)

### 7. Make Tests Deterministic
Tests should always produce the same result:
- Don't rely on current time (use fixed timestamps)
- Don't use random values (use fixed test data)
- Clean up after tests

## Practice Exercises

Try implementing these features using TDD:

### Exercise 1: Add Idea Status
1. Add a `status` field to Idea (Draft, InProgress, Completed)
2. Write tests first
3. Implement the feature
4. Add validation to ensure status transitions are valid

### Exercise 2: Idea Search
1. Implement a function to search ideas by title
2. Write tests for exact match, partial match, case-insensitive
3. Implement the search logic

### Exercise 3: Idea Tags
1. Add a function to find all unique tags
2. Add a function to filter ideas by tag
3. Write tests first, then implement

## Resources

- [Rust Book - Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Dioxus Testing Docs](https://dioxuslabs.com/learn/0.7/testing)
- [TDD by Example (Kent Beck)](https://www.amazon.com/Test-Driven-Development-Kent-Beck/dp/0321146530)

## Next Steps

1. Run the existing tests: `cargo test --features server`
2. Try modifying a test to make it fail
3. Fix the test by changing the implementation
4. Pick a practice exercise and try TDD yourself!

Happy Test-Driven Development! 🦀✅
