use quote::__private::TokenStream;

use crate::{schema::Schema, schema_type::SchemaType};

pub mod class;
pub mod property;

pub fn to_token_stream(schema: &Schema) -> TokenStream {
	match schema.schema_type {
		SchemaType::Property => property::to_token_stream(schema),
		SchemaType::Class => class::to_token_stream(schema),
	}
}
