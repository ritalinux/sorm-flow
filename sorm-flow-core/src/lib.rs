use async_trait::async_trait;
pub use async_trait::async_trait as async_trait_export;
pub use surrealdb;
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{engine::any::Any, RecordId, Surreal};

pub mod query;
pub use query::QueryBuilder;

#[async_trait]
pub trait SormRelation: Serialize + DeserializeOwned + Sized + Send + Sync + 'static {
    fn table_name() -> &'static str;
}

#[async_trait]
pub trait SormEntity: Serialize + DeserializeOwned + Sized + Send + Sync + 'static {
    fn table_name() -> &'static str;
    fn id(&self) -> Option<RecordId>;

    async fn find(db: &Surreal<Any>, id: &str) -> surrealdb::Result<Option<Self>> {
        db.select((Self::table_name(), id)).await
    }

    async fn all(db: &Surreal<Any>) -> surrealdb::Result<Vec<Self>> {
        db.select(Self::table_name()).await
    }

    async fn save(self, db: &Surreal<Any>) -> surrealdb::Result<Option<Self>> {
        if let Some(id) = self.id() {
            db.update(id).content(self).await
        } else {
            db.create(Self::table_name()).content(self).await
        }
    }

    fn query(db: &Surreal<Any>) -> QueryBuilder<'_, Self> {
        QueryBuilder::new(db)
    }

    async fn relate<R: SormEntity>(
        &self,
        db: &Surreal<Any>,
        edge_table: &str,
        target: &R,
    ) -> surrealdb::Result<()> {
        // Correção: Usamos Error::Api com uma string formatada para contornar o problema de tipo
        let from = self.id().ok_or_else(|| surrealdb::Error::Api(surrealdb::error::Api::InternalError("Source ID missing".into())))?;
        let to = target.id().ok_or_else(|| surrealdb::Error::Api(surrealdb::error::Api::InternalError("Target ID missing".into())))?;
        
        let sql = format!("RELATE $from->{}->$to", edge_table);
        
        db.query(sql)
            .bind(("from", from))
            .bind(("to", to))
            .await?;
            
        Ok(())
    }

    /// Remove o registro atual do banco de dados.
    /// Retorna o objeto deletado em caso de sucesso.
    async fn delete(self, db: &Surreal<Any>) -> surrealdb::Result<Option<Self>> {
        // Recupera o ID ou retorna erro se o objeto não tiver um (não persistido)
        let id = self.id().ok_or_else(|| {
            surrealdb::Error::Api(surrealdb::error::Api::InternalError(
                "Cannot delete a record without an ID".into(),
            ))
        })?;

        // Executa o DELETE no SurrealDB
        db.delete(id).await
    }

    /// Método estático para deletar por ID sem precisar da instância
    async fn delete_by_id(db: &Surreal<Any>, id: &str) -> surrealdb::Result<Option<Self>> {
        db.delete((Self::table_name(), id)).await
    }
}
