# Dioxus + SurrealDB Fullstack Template

A production-ready template for building fullstack web applications with **Dioxus** (Rust fullstack framework) and **SurrealDB** (multi-model database). Perfect for quickly spinning up MVPs and proof-of-concepts.

## Features

- **🦀 Full Rust Stack** - Type-safe from database to UI
- **⚡ Server Functions** - RPC-style API endpoints with zero boilerplate
- **🎨 Component-Based UI** - Reactive components with built-in state management
- **🗄️ SurrealDB Integration** - Modern database with embedded (dev) and remote (prod) support
- **🛣️ File-Based Routing** - Declarative routing with layouts
- **📦 Asset Pipeline** - CSS/JS minification and bundling
- **🔄 Hot Reload** - Fast development cycle

## Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | Dioxus (compiles to WebAssembly) |
| Backend | Dioxus Server Functions |
| Database | SurrealDB (RocksDB in dev) |
| Language | Rust (100%) |

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started): `cargo install dioxus-cli`

### Use This Template

#### Option 1: GitHub Template
1. Click "Use this template" on GitHub
2. Clone your new repository
3. Follow setup steps below

#### Option 2: Manual Clone
```bash
git clone https://github.com/verystochastic/dioxus-surrealdb-template my-new-project
cd my-new-project
rm -rf .git
git init
```

### Setup

1. **Rename your project** in `Cargo.toml`:
   ```toml
   [package]
   name = "my-new-project"  # Change this
   version = "0.1.0"
   authors = ["Your Name <your.email@example.com>"]
   ```

2. **Update app title** in `Dioxus.toml`:
   ```toml
   [web.app]
   title = "My New Project"  # Change this
   ```

3. **Install dependencies**:
   ```bash
   cargo build
   ```

4. **Run development server**:
   ```bash
   dx serve --platform web
   ```

5. **Open browser**: http://localhost:8080

## Project Structure

```
.
├── src/
│   ├── main.rs                 # App entry point + routing
│   ├── db.rs                   # Data models + database client
│   ├── server_functions.rs     # API endpoints (server functions)
│   ├── components/             # Reusable UI components
│   │   ├── mod.rs
│   │   ├── idea_form.rs       # Example: Form component
│   │   └── idea_list.rs       # Example: Data-fetching component
│   └── views/                  # Page-level components
│       ├── mod.rs
│       ├── home.rs            # Home page
│       ├── navbar.rs          # Layout wrapper
│       └── blog.rs            # Example dynamic route
├── assets/
│   ├── favicon.ico
│   └── styling/
│       ├── main.css           # Global styles
│       └── idea_form.css      # Component-specific styles
├── Cargo.toml                 # Rust dependencies
├── Dioxus.toml               # Dioxus configuration
└── ideas.db/                  # Local database (gitignored)
```

## Customization Guide

### 1. Define Your Data Model

Edit `src/db.rs`:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct YourModel {
    pub id: Option<String>,
    pub field1: String,
    pub field2: i32,
}
```

### 2. Create Server Functions

Edit `src/server_functions.rs`:
```rust
#[post("/api/your-endpoint")]
pub async fn your_function(param: String) -> Result<YourModel> {
    #[cfg(feature = "server")]
    {
        let db = get_db().await;
        // Your database logic here
    }
}
```

### 3. Build UI Components

Create in `src/components/`:
```rust
#[component]
pub fn YourComponent() -> Element {
    rsx! {
        div { "Your UI here" }
    }
}
```

### 4. Add Routes

Edit `src/main.rs`:
```rust
#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/your-route")]
        YourPage {},
}
```

## Development Workflow

### Run Development Server
```bash
dx serve --platform web
```
Hot reload enabled by default.

### Build for Production
```bash
dx build --platform web --release
```
Outputs to `dist/` directory.

### Run Tests
```bash
cargo test
```

### Format Code
```bash
cargo fmt
```

### Check for Issues
```bash
cargo clippy
```

## Database Configuration

### Development (Local)
Uses embedded RocksDB (file-based). Data stored in `ideas.db/` directory.

### Production (Remote Database)

To deploy with a networked database, update `src/db.rs`:

```rust
// Replace RocksDB connection with remote SurrealDB
use surrealdb::engine::remote::ws::{Client, Ws};

pub async fn get_db() -> &'static Surreal<Client> {
    DB.get_or_init(|| async {
        let db = Surreal::new::<Ws>(env::var("DATABASE_URL")?)
            .await?;

        db.signin(Root {
            username: &env::var("DB_USER")?,
            password: &env::var("DB_PASS")?,
        }).await?;

        db.use_ns("your_ns").use_db("your_db").await?;
        db
    }).await
}
```

Update `Cargo.toml`:
```toml
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
surrealdb = { version = "2.1", features = ["protocol-ws"] }
```

## Deployment

### Recommended Platforms

| Platform | Best For | Rust Support |
|----------|----------|--------------|
| [Shuttle.rs](https://shuttle.rs) | Rust-native apps | Native |
| [Fly.io](https://fly.io) | Global edge | Docker |
| [Railway](https://railway.app) | Simple deploys | Docker |

### Deploy to Shuttle.rs (Easiest)

1. Install Shuttle CLI:
   ```bash
   cargo install cargo-shuttle
   ```

2. Initialize:
   ```bash
   cargo shuttle init
   ```

3. Deploy:
   ```bash
   cargo shuttle deploy
   ```

### Deploy to Fly.io

1. Create `Dockerfile`:
   ```dockerfile
   FROM rust:1.75 as builder
   WORKDIR /app
   COPY . .
   RUN cargo build --release --features server,web

   FROM debian:bookworm-slim
   COPY --from=builder /app/target/release/your-app /usr/local/bin/
   CMD ["your-app"]
   ```

2. Deploy:
   ```bash
   fly launch
   fly deploy
   ```

## Example: Idea Tracker (Included)

This template includes a simple idea tracker as a reference implementation:

- **Model**: `Idea` with title, description, tags
- **Server Functions**: `submit_idea_server()`, `get_all_ideas_server()`
- **Components**: `IdeaForm`, `IdeaList`
- **Features**:
  - Form submission with validation
  - Real-time list updates
  - Tag parsing from comma-separated input

Feel free to delete and replace with your own implementation.

## Architecture Patterns

### Server Functions
```rust
// Define once, call from client like a local async function
#[post("/api/endpoint")]
pub async fn my_function(param: String) -> Result<Data> {
    // Server-only code
}

// Client usage
let data = my_function("test".to_string()).await?;
```

### Component State
```rust
// Local reactive state
let mut count = use_signal(|| 0);

// Update state
count.set(count() + 1);

// Read in JSX
rsx! { div { "{count}" } }
```

### Parent-Child Communication
```rust
// Parent
#[component]
fn Parent() -> Element {
    let mut trigger = use_signal(|| 0);

    rsx! {
        Child { on_event: move |_| trigger += 1 }
    }
}

// Child
#[component]
fn Child(on_event: EventHandler<()>) -> Element {
    rsx! {
        button { onclick: move |_| on_event.call(()), "Click" }
    }
}
```

## Common Tasks

### Add a New Page
1. Create component in `src/views/my_page.rs`
2. Add to `src/views/mod.rs`: `pub use my_page::MyPage;`
3. Add route to `src/main.rs`: `#[route("/my-page")] MyPage {}`

### Add a New Component
1. Create in `src/components/my_component.rs`
2. Export in `src/components/mod.rs`: `pub use my_component::MyComponent;`
3. Use in any view: `MyComponent {}`

### Add Global CSS
1. Create CSS file in `assets/styling/`
2. Import in component/view:
   ```rust
   const MY_CSS: Asset = asset!("/assets/styling/my_styles.css");

   rsx! {
       document::Link { rel: "stylesheet", href: MY_CSS }
   }
   ```

## Troubleshooting

### "Database locked" error
Stop all running instances: `pkill -f december`

### Hot reload not working
Restart dev server: `dx serve --platform web`

### WASM compilation errors
Clear cache: `cargo clean && dx serve`

### Port already in use
Kill process on 8080: `lsof -ti:8080 | xargs kill -9`

## Resources

- [Dioxus Documentation](https://dioxuslabs.com/)
- [SurrealDB Documentation](https://surrealdb.com/docs)
- [Rust Book](https://doc.rust-lang.org/book/)
- [This template's architecture guide](./ARCHITECTURE.md) *(coming soon)*

## Contributing

Found an issue or want to improve this template? PRs welcome!

## License

MIT License - use freely for personal or commercial projects.

---

**Happy Building!** 🚀

For questions or feedback, open an issue on GitHub.
