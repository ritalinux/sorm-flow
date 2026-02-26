# TRD - Technical Requirements Document: sorm-flow 🌊

## 1. Document Purpose
This document outlines the technical architecture, design patterns, and implementation constraints of **sorm-flow**, a fluent ORM for SurrealDB built in Rust. It serves as a guide for both human maintainers and AI assistants.

## 2. System Architecture

### 2.1 Workspace Structure
The project uses a Cargo Workspace to separate concerns and handle Rust's `proc-macro` limitations:
- **`sorm-flow` (Facade):** The entry point. Re-exports macros and core traits for end-user ergonomics.
- **`sorm-flow-core` (Engine):** Contains the `SormEntity` trait, `QueryBuilder` logic, and SurrealDB client integrations.
- **`sorm-flow-macros` (CodeGen):** Procedural macros for `#[derive(Model)]` and `#[derive(Relation)]`.

### 2.2 Core Traits (`SormEntity`)
The backbone of the ORM. Every model must implement `SormEntity` to gain CRUD capabilities.
- **Requirement:** Must support `async/await` via `async_trait`.
- **Requirement:** Must enforce `Serialize + DeserializeOwned + Sync + Send + 'static`.
- **Key Methods:** `save()`, `find()`, `all()`, `delete()`, `query()`, `relate()`.

## 3. Database Integration (SurrealDB 2.x)

### 3.1 Record ID Management
SurrealDB uses unique `RecordId` structures (`table:id`).
- **Policy:** The ORM identifies the primary key by a field named `id` of type `Option<RecordId>`.
- **Complexity:** Supports complex IDs (Arrays/Objects) through the `id` field without duplicating data in the JSON body (Schemafull compliance).

### 3.2 Graph Engine
- **Creation:** Uses the `RELATE` statement via parameterized queries to ensure compatibility across all SurrealDB storage engines (TiKV, RocksDB, Memory).
- **Traversal:** The `QueryBuilder` supports graph walking using the `from_graph` method, translating Rust calls to `SELECT * FROM node->edge->node`.

## 4. Query Builder Design

### 4.1 Type Safety & PhantomData
Uses `PhantomData<T>` to bind query results to the specific Model type at compile-time, ensuring that `fetch()` always returns the correct struct.

### 4.2 Parameterization (Security)
All filters must use **Bindings** (`$param`). Manual string concatenation for values is strictly forbidden to prevent SQL Injection.
- **Implementation:** `bindings: HashMap<String, Value>`.

### 4.3 Clause Ordering
The Builder must respect SurrealQL's strict clause order:
`SELECT` -> `FROM` -> `WHERE` -> `ORDER BY` -> `LIMIT` -> `START` -> `FETCH`.

## 5. Technical Constraints & Style

### 5.1 Error Handling
- Use `surrealdb::Result` for database operations.
- Custom errors should be wrapped in `surrealdb::Error::Api` for consistency with the SDK.

### 5.2 Async Patterns
- Given current Rust limitations, `async_trait` is required for trait methods.
- The `SormEntity` trait requires `Self: 'static` to ensure safety in background async tasks.

## 6. Future Extensibility (Roadmap)
- **Migrations:** A system to sync Rust structs with `DEFINE TABLE/FIELD` statements.
- **Aggregations:** Support for `count()`, `sum()`, and `group_by()` in the Builder.
- **Events:** Integration with SurrealDB events (`DEFINE EVENT`).

---
**Revision:** 1.0 (Feb 2026)  
**Assistance:** Developed with AI / Validated by Humans.
