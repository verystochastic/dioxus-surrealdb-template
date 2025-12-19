# Template Usage Guide

Quick reference for using this template to build new projects.

## Initial Setup Checklist

When starting a new project from this template:

- [ ] **Clone or use GitHub template**
- [ ] **Remove existing git history** (if manually cloned): `rm -rf .git && git init`
- [ ] **Update `Cargo.toml`**:
  - [ ] Change `name = "december"` to your project name
  - [ ] Update `authors`
  - [ ] Update `version` if desired
- [ ] **Update `Dioxus.toml`**:
  - [ ] Change `title = "december"` to your app name
- [ ] **Update `README.md`**:
  - [ ] Replace template description with your project description
  - [ ] Update repository URLs
- [ ] **Run initial build**: `cargo build`
- [ ] **Test dev server**: `dx serve --platform web`

## Cleaning Out Example Code

This template includes an "Idea Tracker" example. To start fresh:

### 1. Clean up `src/db.rs`
Replace the `Idea` model with your own:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct YourModel {
    pub id: Option<String>,
    // Your fields here
}
```

### 2. Clean up `src/server_functions.rs`
Replace example functions:
```rust
#[post("/api/your-endpoint")]
pub async fn your_function() -> Result<YourType> {
    // Your logic
}
```

### 3. Clean up components
Delete or replace:
- `src/components/idea_form.rs`
- `src/components/idea_list.rs`
- `src/components/echo.rs` (demo component)
- `src/components/hero.rs` (demo component)

Update `src/components/mod.rs` to remove references.

### 4. Clean up views
Simplify `src/views/home.rs`:
```rust
#[component]
pub fn Home() -> Element {
    rsx! {
        div { "Welcome to your new app!" }
    }
}
```

Delete `src/views/blog.rs` if not needed and remove from routes.

### 5. Clean up assets
Replace/remove:
- `assets/styling/idea_form.css`
- Update `assets/styling/main.css` with your styles

## Common Patterns Reference

### Adding a Database Model

1. Define in `src/db.rs`:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Product {
    pub id: Option<String>,
    pub name: String,
    pub price: f64,
}

#[cfg(feature = "server")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductRecord {
    pub id: Option<surrealdb::sql::Thing>,
    pub name: String,
    pub price: f64,
}

#[cfg(feature = "server")]
impl From<ProductRecord> for Product {
    fn from(record: ProductRecord) -> Self {
        Product {
            id: record.id.map(|thing| thing.to_string()),
            name: record.name,
            price: record.price,
        }
    }
}
```

2. Create server functions in `src/server_functions.rs`:
```rust
#[post("/api/products/create")]
pub async fn create_product(name: String, price: f64) -> Result<Product> {
    #[cfg(feature = "server")]
    {
        use crate::db::{server::get_db, ProductRecord};

        let product = ProductRecord {
            id: None,
            name,
            price,
        };

        let db = get_db().await;
        let created: Option<ProductRecord> = db
            .create("products")
            .content(product)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(created.unwrap().into())
    }
    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new("Server-only function"))
    }
}

#[post("/api/products/list")]
pub async fn list_products() -> Result<Vec<Product>> {
    #[cfg(feature = "server")]
    {
        use crate::db::{server::get_db, ProductRecord};

        let db = get_db().await;
        let products: Vec<ProductRecord> = db
            .select("products")
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(products.into_iter().map(|r| r.into()).collect())
    }
    #[cfg(not(feature = "server"))]
    {
        Err(ServerFnError::new("Server-only function"))
    }
}
```

### Adding a Form Component

```rust
use dioxus::prelude::*;

#[component]
pub fn ProductForm(on_success: EventHandler<()>) -> Element {
    let mut name = use_signal(|| String::new());
    let mut price = use_signal(|| String::new());
    let mut is_submitting = use_signal(|| false);

    rsx! {
        form {
            onsubmit: move |e| async move {
                e.prevent_default();
                is_submitting.set(true);

                let price_val: f64 = price().parse().unwrap_or(0.0);

                match create_product(name(), price_val).await {
                    Ok(_) => {
                        name.set(String::new());
                        price.set(String::new());
                        on_success.call(());
                    }
                    Err(e) => {
                        // Handle error
                    }
                }

                is_submitting.set(false);
            },

            input {
                r#type: "text",
                value: "{name}",
                oninput: move |e| name.set(e.value()),
                placeholder: "Product name"
            }

            input {
                r#type: "number",
                value: "{price}",
                oninput: move |e| price.set(e.value()),
                placeholder: "Price"
            }

            button {
                r#type: "submit",
                disabled: is_submitting(),
                "Create Product"
            }
        }
    }
}
```

### Adding a List Component

```rust
use dioxus::prelude::*;
use crate::server_functions::list_products;

#[component]
pub fn ProductList(refresh_trigger: Signal<u32>) -> Element {
    let products = use_resource(move || async move {
        refresh_trigger.read(); // Re-run when trigger changes
        list_products().await
    });

    rsx! {
        div {
            h2 { "Products" }

            match products.read().as_ref() {
                Some(Ok(items)) => rsx! {
                    ul {
                        for product in items {
                            li {
                                key: "{product.id:?}",
                                "{product.name} - ${product.price}"
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! {
                    div { "Error: {e}" }
                },
                None => rsx! {
                    div { "Loading..." }
                }
            }
        }
    }
}
```

### Adding a New Route

1. Define route in `src/main.rs`:
```rust
#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/products")]
        Products {},
}
```

2. Create view in `src/views/products.rs`:
```rust
use dioxus::prelude::*;

#[component]
pub fn Products() -> Element {
    rsx! {
        div { "Products page" }
    }
}
```

3. Export from `src/views/mod.rs`:
```rust
pub use products::Products;
```

4. Import in `src/main.rs`:
```rust
use views::{Home, Navbar, Products};
```

## Development Tips

### Hot Reload Best Practices
- Keep `dx serve` running while developing
- Save files to trigger automatic recompilation
- Check terminal for compilation errors

### Debugging
- Use `println!` or `dbg!` for quick debugging
- Check browser console for client-side errors
- Check terminal for server-side errors

### Testing New Features
1. Start small - test each component individually
2. Use the example components as reference
3. Build incrementally - don't change everything at once

## Production Deployment Checklist

Before deploying:

- [ ] Update database connection to use remote DB
- [ ] Set up environment variables (use `.env.example` as reference)
- [ ] Test production build: `dx build --platform web --release`
- [ ] Add error handling to all server functions
- [ ] Add input validation
- [ ] Set up CORS if needed
- [ ] Configure HTTPS/SSL
- [ ] Set up monitoring/logging
- [ ] Create backup strategy for database

## Getting Help

- Dioxus Discord: https://discord.gg/XgGxMSkvUM
- Dioxus Docs: https://dioxuslabs.com/
- SurrealDB Discord: https://discord.com/invite/surrealdb
- Rust Forum: https://users.rust-lang.org/

## Next Steps

After initial setup:

1. **Design your data model** - What entities do you need?
2. **Plan your routes** - What pages will your app have?
3. **Build core components** - Forms, lists, navigation
4. **Implement business logic** - Server functions for CRUD operations
5. **Style your app** - Update CSS files
6. **Test thoroughly** - Try breaking it!
7. **Deploy** - Ship it to production

Happy coding! 🚀
