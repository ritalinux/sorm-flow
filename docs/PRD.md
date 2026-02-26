# PRD - Product Requirements Document: sorm-flow 🌊

## 1. Executive Summary
**sorm-flow** is a high-level Object-Relational Mapper (ORM) for SurrealDB. It aims to provide the most ergonomic, "fluent" development experience in the Rust ecosystem, allowing developers to treat SurrealDB's multi-model capabilities (Document, Graph, Relational) as native Rust objects.

## 2. Target Audience
- **Rust Backend Developers:** Who want to avoid writing raw SurrealQL strings.
- **System Architects:** Looking for a type-safe way to manage complex Graph data.
- **Microservices Teams:** Who need a consistent data access layer (DAL) across multiple services.

## 3. Problem Statement
SurrealDB is powerful but its Rust SDK is low-level. Developers often face:
- **String-based Queries:** Prone to typos and SQL injection if not handled carefully.
- **Manual Mapping:** Complexity in converting Graph results into nested Rust structs.
- **Boilerplate:** Repetitive code for basic CRUD operations.

## 4. User Stories
- **US1 (Basic CRUD):** As a dev, I want to call `.save()` on a struct and have the ORM decide between CREATE (new) or UPDATE (existing).
- **US2 (Fluent Filtering):** As a dev, I want to chain `.filter().order_by().limit()` to build queries that feel like native Rust.
- **US3 (Graph Navigation):** As a dev, I want to follow relationships (edges) using a simple method call instead of complex JOINs.
- **US4 (Relationship Fetching):** As a dev, I want to "eager load" related records (FETCH) so I don't hit the N+1 query problem.

## 5. Functional Requirements (MVP)
- **R1 (Model Derivation):** Automated implementation of database traits via `#[derive(Model)]`.
- **R2 (Query Builder):** A stateful builder that generates valid SurrealQL.
- **R3 (Graph Support):** First-class support for `RELATE` and graph traversal.
- **R4 (Type Safety):** All query results must be typed at compile-time using Rust Generics.

## 6. Non-Functional Requirements
- **Performance:** Minimal overhead over the native SurrealDB driver (zero-cost abstractions where possible).
- **Reliability:** Built with AI assistance but strictly validated by human-driven integration tests.
- **Simplicity:** Documentation-first approach with clear "Usage Examples".

## 7. Success Metrics
- **Ease of Use:** A new developer should be able to perform a Graph Traversal query in less than 10 lines of code.
- **Adoption:** Become a go-to community recommendation for SurrealDB + Rust projects.

## 8. Development Methodology
This project utilizes **AI-Assisted Development**. 
- **AI Role:** Architecture drafting, boilerplate generation, and initial logic implementation.
- **Human Role:** Scenario testing, security auditing, and final architectural approval.

---
**Status:** In Development (v0.1.x)  
**Author:** votu-operations Team
