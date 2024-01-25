use std::{collections::BinaryHeap, path::Path, str::FromStr, sync::Mutex};

use derivative::Derivative;
use indicatif::{MultiProgress, ProgressBar};
use oxigraph::store::Store;
use quote::{__private::TokenStream, quote};
use rayon::prelude::*;
use strum::IntoEnumIterator;

use crate::{
	schema::Schema,
	schema_org_crate::SchemaOrgCrate,
	schema_type::SchemaType,
	sparql::{SchemaQueries, SchemaQuerySolution},
};

#[derive(Debug, Clone, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
struct SchemaModuleInfo {
	pub name: String,
}

impl From<&Schema> for SchemaModuleInfo {
	fn from(value: &Schema) -> Self {
		Self {
			name: value.module_name(),
		}
	}
}

trait ToModuleString {
	fn to_module_string(&self) -> String;
}
impl ToModuleString for &[&SchemaModuleInfo] {
	fn to_module_string(&self) -> String {
		let schema_mods_and_pub_uses = self.iter().map(|schema| {
			let module_name = TokenStream::from_str(&format!("r#{}", schema.name)).unwrap();
			quote!(
				mod #module_name;
				pub use self::#module_name::*;
			)
		});
		quote!(
			use super::*;
			#(#schema_mods_and_pub_uses)*
		)
		.to_string()
	}
}

fn to_module_string(prefix: Option<TokenStream>, submodules: &[impl AsRef<str>]) -> String {
	let schema_mods_and_pub_uses = submodules.iter().map(|submodule| {
		let module_name = TokenStream::from_str(&format!("r#{}", submodule.as_ref())).unwrap();
		quote!(
			mod #module_name;
			pub use self::#module_name::*;
		)
	});
	quote!(
		#prefix

		#(#schema_mods_and_pub_uses)*
	)
	.to_string()
}

pub fn pretty_please(str: &str) -> String {
	let syntax_tree = syn::parse_file(str).unwrap();
	prettyplease::unparse(&syntax_tree)
}

fn write_parent_module(
	prefix: Option<TokenStream>,
	submodules: &[impl AsRef<str>],
	module_file: &Path,
) {
	std::fs::write(
		module_file,
		pretty_please(&to_module_string(prefix, submodules)),
	)
	.unwrap();
}

pub fn write(store: &Store, multi_progress: &MultiProgress) {
	for schema_org_crate in SchemaOrgCrate::iter() {
		let schemas_dir = schema_org_crate.schemas_dir();
		std::fs::remove_dir_all(&schemas_dir).unwrap();
		std::fs::create_dir(&schemas_dir).unwrap();
		for schema_type in schema_org_crate.schema_types() {
			std::fs::create_dir(schema_org_crate.schema_type_dir(schema_type)).unwrap();
		}
	}

	let schemas = store.get_schemas();

	let all_schema_modules = Mutex::new(BinaryHeap::<String>::new());
	let class_schema_modules = Mutex::new(BinaryHeap::<String>::new());
	let property_schema_modules = Mutex::new(BinaryHeap::<String>::new());

	let handle_write = |solution: SchemaQuerySolution| {
		let schema = Schema::from_solution(store, solution);
		let schema_module = schema.module_name();
		match schema.schema_type {
			SchemaType::Property => property_schema_modules
				.lock()
				.unwrap()
				.push(schema_module.clone()),
			SchemaType::Class => class_schema_modules
				.lock()
				.unwrap()
				.push(schema_module.clone()),
		};
		all_schema_modules.lock().unwrap().push(schema_module);
		for schema_org_crate in SchemaOrgCrate::iter() {
			schema.write_module(&schema_org_crate);
		}
	};

	let bar = multi_progress.add(ProgressBar::new(schemas.len() as u64));
	schemas.into_par_iter().for_each(|solution| {
		handle_write(solution);
		bar.inc(1);
	});

	write_parent_module(
		Some(quote!(
			#![allow(unused)]
			#![allow(deprecated)]
		)),
		all_schema_modules
			.into_inner()
			.unwrap()
			.into_sorted_vec()
			.iter()
			.collect::<Vec<_>>()
			.as_slice(),
		&SchemaOrgCrate::Constants.schemas_file(),
	);
	write_parent_module(
		Some(quote!(
			#![allow(unused)]
			#![allow(deprecated)]
		)),
		SchemaOrgCrate::Traits
			.schema_types()
			.iter()
			.map(|schema_type| schema_type.parent_module_name().to_string())
			.collect::<Vec<_>>()
			.as_slice(),
		&SchemaOrgCrate::Traits.schemas_file(),
	);
	write_parent_module(
		None,
		class_schema_modules
			.into_inner()
			.unwrap()
			.into_sorted_vec()
			.as_slice(),
		&SchemaOrgCrate::Traits.schema_type_file(&SchemaType::Class),
	);
	write_parent_module(
		None,
		property_schema_modules
			.into_inner()
			.unwrap()
			.into_sorted_vec()
			.as_slice(),
		&SchemaOrgCrate::Traits.schema_type_file(&SchemaType::Property),
	);
}
