use async_trait::async_trait;
pub use async_trait::async_trait as async_trait_export; // Para a macro usar
pub use surrealdb; // Re-exporta para a macro achar
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{engine::any::Any, sql::RecordId, Surreal};

#[async_trait]
pub trait SormEntity: Serialize + DeserializeOwned + Sized + Send + Sync {
    fn table_name() -> &'static str;
    fn id(&self) -> Option<RecordId>;

    // SELECT * FROM table WHERE id = $id
    async fn find(db: &Surreal<Any>, id: &str) -> surrealdb::Result<Option<Self>> {
        db.select((Self::table_name(), id)).await
    }

    // SELECT * FROM table
    async fn all(db: &Surreal<Any>) -> surrealdb::Result<Vec<Self>> {
        db.select(Self::table_name()).await
    }

    // CREATE ou UPDATE (Estilo SQLAlchemy save)
    async fn save(self, db: &Surreal<Any>) -> surrealdb::Result<Option<Self>> {
        if let Some(id) = self.id() {
            db.update(id).content(self).await
        } else {
            db.create(Self::table_name()).content(self).await
        }
    }
}
