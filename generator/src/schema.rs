use convert_case::{Case, Casing};
use derivative::Derivative;
use oxigraph::store::Store;
use quote::__private::TokenStream;

use crate::{
	deprecated_attribute::DeprecatedAttribute,
	schema_org_crate::SchemaOrgCrate,
	schema_type::SchemaType,
	sparql::{SchemaQueries, SchemaQuerySolution},
	write::pretty_please,
};

mod constants;
mod traits;

#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Schema {
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub iri: String,
	pub name: String,
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub superseded_by: Vec<Schema>,
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub in_attic: bool,
	#[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
	pub schema_type: SchemaType,
}

impl Schema {
	pub fn module_name(&self) -> String {
		self.type_name().to_case(Case::Snake)
	}

	fn type_name(&self) -> String {
		match self.schema_type {
			SchemaType::Property => format!("{}Property", self.name.to_case(Case::UpperCamel)),
			SchemaType::Class => self.name.to_case(Case::UpperCamel),
		}
	}

	pub fn from_solution(store: &Store, solution: SchemaQuerySolution) -> Self {
		let superseded_by = store
			.get_superseded_by(&solution.iri)
			.into_iter()
			.map(|solution| Schema::from_solution(store, solution))
			.collect();
		let schema_type = SchemaType::from_iri(store, &solution.iri);
		Self {
			iri: solution.iri,
			name: map_schema_name(solution.label),
			superseded_by,
			in_attic: solution.in_attic,
			schema_type,
		}
	}

	fn to_token_stream(&self, schema_org_crate: &SchemaOrgCrate) -> TokenStream {
		match schema_org_crate {
			SchemaOrgCrate::Constants => self.to_token_stream_constants(),
			SchemaOrgCrate::Traits => self.to_token_stream_traits(),
		}
	}

	fn to_token_stream_constants(&self) -> TokenStream {
		constants::to_token_stream(self)
	}

	fn to_token_stream_traits(&self) -> TokenStream {
		traits::to_token_stream(self)
	}

	pub fn write_module(&self, schema_org_crate: &SchemaOrgCrate) {
		let mut file_path = schema_org_crate.schema_type_dir(&self.schema_type);
		file_path.push(format!("{}.rs", self.module_name()));
		std::fs::write(
			file_path,
			pretty_please(&self.to_token_stream(schema_org_crate).to_string()),
		)
		.unwrap();
	}
}

impl DeprecatedAttribute for Schema {
	fn in_attic(&self) -> bool {
		self.in_attic
	}

	fn superseded_by(&self) -> &[Schema] {
		self.superseded_by.as_slice()
	}
}

/// Map schema names which are incompatible with rust as identifier.
fn map_schema_name(name: String) -> String {
	match name.as_str() {
		"3DModel" => "Model3D".to_string(),
		_ => name,
	}
}
