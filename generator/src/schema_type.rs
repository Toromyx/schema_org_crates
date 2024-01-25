use oxigraph::store::Store;

use crate::sparql::SchemaQueries;

#[derive(Debug, Clone, PartialEq)]
pub enum SchemaType {
	Property,
	Class,
}

impl SchemaType {
	pub fn from_iri(store: &Store, iri: &str) -> Self {
		if store.is_property(iri) {
			return Self::Property;
		}
		Self::Class
	}

	pub fn parent_module_name(&self) -> &'static str {
		match self {
			SchemaType::Property => "properties",
			SchemaType::Class => "classes",
		}
	}
}
