use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Model)]
pub fn sorm_model_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let table_name = name.to_string().to_lowercase();

    let expanded = quote! {
        // Implementa a trait usando o caminho da facade principal
        #[sorm_flow::core::async_trait_export]
        impl sorm_flow::core::SormEntity for #name {
            fn table_name() -> &'static str {
                #table_name
            }

            fn id(&self) -> Option<sorm_flow::core::surrealdb::sql::RecordId> {
                self.id.clone()
            }
        }
    };

    TokenStream::from(expanded)
}
