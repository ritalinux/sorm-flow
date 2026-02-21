// Re-exporta a Macro
pub use sorm_flow_macros::Model;

// Re-exporta o Core com um nome amigável para a Macro
pub mod core {
    pub use sorm_flow_core::*;
}

// Facilita o uso: use sorm_flow::prelude::*;
pub mod prelude {
    pub use crate::Model;
    pub use crate::core::SormEntity;
    pub use crate::core::surrealdb::sql::RecordId;
}
