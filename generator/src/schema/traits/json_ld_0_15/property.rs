use quote::{__private::TokenStream, quote};

use crate::schema::{
	constants::{const_iri_http, const_iri_https},
	traits::base::property::{function_name, trait_name},
	Schema,
};

pub fn to_token_stream(schema: &Schema) -> TokenStream {
	let trait_name = trait_name(schema);
	let function_name = function_name(schema);
	let const_iri_http = const_iri_http(schema);
	let const_iri_https = const_iri_https(schema);
	quote!(
		use schema_org_constants::SchemaOrgNamespace;

		impl crate::#trait_name for crate::json_ld_0_15::JsonLdStore {
			type IdType = json_ld_0_15::ValidId;
			type PropertyType = rdf_types_0_15::Object;

			fn #function_name(&self, id: &Self::IdType) -> Vec<&Self::PropertyType> {
				self.get_property(
					id,
					match self.namespace() {
						SchemaOrgNamespace::Http => schema_org_constants::#const_iri_http,
						SchemaOrgNamespace::Https => schema_org_constants::#const_iri_https,
					},
				)
			}
		}
	)
}
