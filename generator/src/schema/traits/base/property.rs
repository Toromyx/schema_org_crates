use std::str::FromStr;

use convert_case::{Case, Casing};
use quote::{__private::TokenStream, quote};

use crate::{deprecated_attribute::DeprecatedAttribute, doc_lines::DocLines, schema::Schema};

pub fn trait_name(schema: &Schema) -> TokenStream {
	TokenStream::from_str(&format!(
		"Get{}",
		schema.type_name().to_case(Case::UpperCamel)
	))
	.unwrap()
}

pub fn function_name(schema: &Schema) -> TokenStream {
	TokenStream::from_str(&format!("get_{}", schema.type_name().to_case(Case::Snake))).unwrap()
}

pub fn to_token_stream(schema: &Schema) -> TokenStream {
	let doc_lines = schema.doc_lines_token_stream();
	let deprecated_attribute = schema.deprecated_attribute();
	let trait_name = trait_name(schema);
	let function_name = function_name(schema);
	quote!(
		#doc_lines
		#deprecated_attribute
		pub trait #trait_name {
			type IdType;
			type PropertyType;
			#deprecated_attribute
			fn #function_name(&self, id: &Self::IdType) -> Vec<&Self::PropertyType>;
		}
	)
}
