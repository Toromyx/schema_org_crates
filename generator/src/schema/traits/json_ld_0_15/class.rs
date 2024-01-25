use quote::{__private::TokenStream, quote};

use crate::schema::{
	constants::{const_iri_http, const_iri_https},
	traits::base::class::{function_name, trait_name},
	Schema,
};

pub fn to_token_stream(schema: &Schema) -> TokenStream {
	let trait_name = trait_name(schema);
	let function_name = function_name(schema);
	let const_iri_http = const_iri_http(schema);
	let const_iri_https = const_iri_https(schema);
	quote!(
		use schema_org_constants::SchemaOrgNamespace;

		impl super::#trait_name for crate::json_ld_0_15::JsonLdStore {
			type IdType = json_ld_0_15::ValidId;

			fn #function_name(&self) -> Vec<&Self::IdType> {
				self.find_schema(
					match self.namespace() {
						SchemaOrgNamespace::Http => schema_org_constants::#const_iri_http,
						SchemaOrgNamespace::Https => schema_org_constants::#const_iri_https,
					},
				)
			}
		}
	)
}
