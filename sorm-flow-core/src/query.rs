use crate::SormEntity;
use surrealdb::{engine::any::Any, Surreal, sql::Value};
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct QueryBuilder<'a, T: SormEntity> {
    pub(crate) db: &'a Surreal<Any>, // Adicionado pub(crate) para visibilidade se necessário
    pub(crate) from_source: String,
    pub(crate) filters: Vec<String>,
    pub(crate) bindings: HashMap<String, Value>,
    pub(crate) fetch_fields: Vec<String>,
    pub(crate) order_field: Option<String>,
    pub(crate) order_direction: Option<String>,
    pub(crate) limit: Option<u32>,
    pub(crate) start: Option<u32>,
    pub(crate) _marker: PhantomData<T>,
}

impl<'a, T: SormEntity> QueryBuilder<'a, T> {
    pub fn new(db: &'a Surreal<Any>) -> Self {
        Self {
            db,
            from_source: T::table_name().to_string(),
            filters: Vec::new(),
            bindings: HashMap::new(),
            fetch_fields: Vec::new(),
            order_field: None,
            order_direction: None,
            limit: None,
            start: None,
            _marker: PhantomData,
        }
    }

    pub fn from_graph(mut self, start_node: &str, traversal: &str) -> Self {
        self.from_source = format!("{}{}", start_node, traversal);
        self
    }

    pub fn filter<V>(mut self, field: &str, operator: &str, value: V) -> Self 
    where 
        V: Into<Value> 
    {
        let param_name = format!("param_{}", self.filters.len());
        self.filters.push(format!("{} {} ${}", field, operator, param_name));
        self.bindings.insert(param_name, value.into());
        self
    }

    pub fn fetch_field(mut self, field: &str) -> Self {
        self.fetch_fields.push(field.to_string());
        self
    }

    pub fn order_by(mut self, field: &str, direction: &str) -> Self {
        self.order_field = Some(field.to_string());
        self.order_direction = Some(direction.to_uppercase());
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn start(mut self, start: u32) -> Self {
        self.start = Some(start);
        self
    }

    pub async fn fetch(self) -> surrealdb::Result<Vec<T>> {
        let mut sql = format!("SELECT * FROM {}", self.from_source);
        
        if !self.filters.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&self.filters.join(" AND "));
        }

        if let (Some(field), Some(dir)) = (self.order_field, self.order_direction) {
            sql.push_str(&format!(" ORDER BY {} {}", field, dir));
        }

        if let Some(limit) = self.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(start) = self.start {
            sql.push_str(&format!(" START {}", start));
        }

        if !self.fetch_fields.is_empty() {
            sql.push_str(" FETCH ");
            sql.push_str(&self.fetch_fields.join(", "));
        }

        let mut query = self.db.query(sql);
        for (name, value) in self.bindings {
            query = query.bind((name, value));
        }

        let mut response = query.await?;
        Ok(response.take(0)?)
    }
}
