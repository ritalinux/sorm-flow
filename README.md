# sorm-flow 🌊

[![License: MIT](https://img.shields.io)](https://opensource.org)
[![Rust](https://img.shields.io)](https://www.rust-lang.org)
[![SurrealDB](https://img.shields.io)](https://surrealdb.com)

**sorm-flow** is a fluent, type-safe ORM for SurrealDB, heavily inspired by the ergonomics of SQLAlchemy and built specifically for the Rust ecosystem.

---

**Not ready for production**

---

## 📚 Documentation

Detailed technical and product specifications are available in the `docs` directory:

- [**PRD (Product Requirements Document)**](docs/PRD.md): Outlines the project vision, user stories, and high-level functional requirements.
- [**TRD (Technical Requirements Document)**](docs/TRD.md): Deep dive into the architectural decisions, trait designs, and SurrealDB 2.x integration patterns.

## ⚠️ Project Status & AI-Assisted Development

> **Important:** This project is being actively developed with the assistance of **Advanced Artificial Intelligence** and is strictly validated through **human testing in real-world scenarios**. Every architectural decision and line of code is peer-reviewed by human developers to ensure production-grade reliability and idiomatic Rust patterns.

---

## 🚀 Vision

The goal of **sorm-flow** is to bridge the gap between the ultra-flexible SurrealQL and Rust's type system. We want to provide a "Flow" experience where developers can interact with their database using pure Rust structures and intuitive method chaining, without sacrificing the powerful features of SurrealDB like Graph relations and Schemafull constraints.

## ✨ Current Features

- [x] **Type-Safe Models**: Define your schema using standard Rust structs with `#[derive(Model)]`.
- [x] **Fluent Query Builder**: Construct complex queries without writing a single line of raw SQL.
- [x] **Graph Relations**: Native support for `RELATE` operations (Record-to-Record arrows).
- [x] **Advanced Traversal**: Navigate through Graph edges directly from the Query Builder.
- [x] **Full CRUD Support**: Standardized `save`, `find`, `all`, and `delete` methods.
- [x] **Pagination & Ordering**: Built-in `limit`, `start` (offset), and `order_by` support.
- [x] **Fetched Relations**: Resolve Record IDs into full objects using the `FETCH` clause effortlessly.

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sorm-flow = "0.1.0"
```

## 🛠 Usage Examples

### Define your Model
```rust
use sorm_flow::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Model, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<RecordId>,
    pub name: String,
    pub active: bool,
}
```

### Basic CRUD
```rust
// Create or Update
let user = User { id: None, name: "Alice".into(), active: true };
let saved_user = user.save(&db).await?;

// Find by ID
let alice = User::find(&db, "alice_id").await?;

// Delete
alice.delete(&db).await?;
```

### Fluent Queries & Graph Traversal
```rust
// Advanced filtering and ordering
let users = User::query(&db)
    .filter("active", "=", true)
    .order_by("name", "ASC")
    .limit(10)
    .fetch()
    .await?;

// Graph Traversal: Get users followed by Alice
let followers = User::query(&db)
    .from_graph("user:alice", "->follows->user")
    .fetch()
    .await?;
```

### Creating Graph Relations
```rust
let alice = User::find(&db, "alice").await?.unwrap();
let bob = User::find(&db, "bob").await?.unwrap();

// RELATE user:alice -> follows -> user:bob
alice.relate(&db, "follows", &bob).await?;
```

## 🗺 Roadmap
- [ ] Support for `GROUP BY` and Aggregations (`count`, `sum`).
- [ ] Automatic Migration System (Schema-as-Code).
- [ ] Expanded Graph macros for Edge data.
- [ ] Comprehensive documentation and integration tests.

## 📄 License
This project is licensed under the **MIT License**.

---
Built with ❤️ by the **sorm-flow** contributors.