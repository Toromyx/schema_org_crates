use std::{
	collections::{HashMap, HashSet},
	fmt::Debug,
};

use iref_2::IriBuf;
use json_ld_0_15::{
	rdf::RDF_TYPE, syntax, syntax::Value, ContextLoader, JsonLdProcessor, Loader, ValidId,
};
use rdf_types_0_15::{
	generator::{Blank, WithMetadata},
	Object, Quad,
};
use schema_org_constants::SchemaOrgNamespace;

#[derive(Debug)]
pub struct JsonLdStore {
	namespace: SchemaOrgNamespace,
	quads: Vec<Quad<ValidId, ValidId>>,
	by_subject: HashMap<String, HashSet<usize>>,
	by_predicate: HashMap<String, HashSet<usize>>,
	by_object: HashMap<String, HashSet<usize>>,
}

impl JsonLdStore {
	pub async fn new<
		JsonLdProcessorType: JsonLdProcessor<IriBuf, ()> + Send + Sync,
		LoaderType: Loader<IriBuf, ()> + ContextLoader<IriBuf, ()> + Send + Sync,
	>(
		json_ld_processor: JsonLdProcessorType,
		loader: &mut LoaderType,
		namespace: Option<SchemaOrgNamespace>,
	) -> Self
	where
		LoaderType::Output: Into<Value>,
		LoaderType::Error: Debug + Send,
		LoaderType::Context: Into<syntax::context::Value<()>>,
		LoaderType::ContextError: Debug + Send,
	{
		let mut generator: WithMetadata<Blank, ()> = Blank::new().with_default_metadata();
		let quads: Vec<_> = json_ld_processor
			.to_rdf(&mut generator, loader)
			.await
			.unwrap()
			.quads()
			.cloned()
			.collect();
		let mut by_subject = HashMap::new();
		let mut by_predicate = HashMap::new();
		let mut by_object = HashMap::new();
		for (index, quad) in quads.iter().enumerate() {
			let add = |string: String,
			           mut by_what: HashMap<String, HashSet<usize>>|
			 -> HashMap<String, HashSet<usize>> {
				let indices = by_what.entry(string).or_default();
				indices.insert(index);
				by_what
			};
			let subject_string = quad.subject().to_string();
			by_subject = add(subject_string, by_subject);
			let predicate_string = quad.predicate().to_string();
			by_predicate = add(predicate_string, by_predicate);
			if let Some(object_iri) = quad.object().as_iri() {
				let object_string = object_iri.to_string();
				by_object = add(object_string, by_object);
			}
		}
		Self {
			namespace: namespace.unwrap_or(SchemaOrgNamespace::Http),
			quads,
			by_subject,
			by_predicate,
			by_object,
		}
	}

	pub fn namespace(&self) -> &SchemaOrgNamespace {
		&self.namespace
	}

	pub fn find_schema(&self, schema_iri: &str) -> Vec<&ValidId> {
		let Some(predicate_indices) = self.by_predicate.get(RDF_TYPE.as_str()) else {
			return vec![];
		};
		let Some(object_indices) = self.by_object.get(schema_iri) else {
			return vec![];
		};
		let indices: HashSet<_> = predicate_indices.intersection(object_indices).collect();
		indices
			.into_iter()
			.filter_map(|index| self.quads.get(*index))
			.map(|quad| quad.subject())
			.collect()
	}

	pub fn get_property(&self, id: &ValidId, property_iri: &str) -> Vec<&Object> {
		let Some(subject_indices) = self.by_subject.get(id.as_str()) else {
			return vec![];
		};
		let Some(predicate_indices) = self.by_predicate.get(property_iri) else {
			return vec![];
		};
		let indices: HashSet<_> = subject_indices.intersection(predicate_indices).collect();
		indices
			.into_iter()
			.filter_map(|index| self.quads.get(*index))
			.map(|quad| quad.object())
			.collect()
	}

	pub fn get_type(&self, id: &ValidId) -> Option<&Object> {
		let types = self.get_property(id, RDF_TYPE.as_str());
		types.first().copied()
	}
}

#[cfg(test)]
mod tests {
	use json_ld_0_15::{syntax::Parse, RemoteDocument, ReqwestLoader};
	use schema_org_constants::HOW_TO_SECTION_IRI_HTTP;

	use super::*;
	use crate::{
		FindNutritionInformationIds, FindRecipeIds, GetCaloriesProperty, GetNameProperty,
		GetRecipeIngredientProperty, GetRecipeInstructionsProperty,
	};

	#[tokio::test]
	async fn test_json_ld_recipe_schema_graph() {
		let schema_graph_text = include_str!("json_ld/tests/recipe_schema_graph.json");
		let input = RemoteDocument::new(
			None,
			Some("application/ld+json".parse().unwrap()),
			Value::parse_str(schema_graph_text, |_| ()).unwrap(),
		);
		let mut loader = ReqwestLoader::new_with_metadata_map(|_, _, _| ());
		let json_ld_store = JsonLdStore::new(input, &mut loader, None).await;
		let recipe_ids = json_ld_store.find_recipe_ids();
		let recipe_id = *recipe_ids.first().unwrap();
		assert_eq!(recipe_id.as_str(), "https://example.test/recipe/#recipe");
		let recipe_names = json_ld_store.get_name_property(recipe_id);
		let recipe_name = *recipe_names.first().unwrap();
		assert_eq!(recipe_name.as_literal().unwrap().as_str(), "Recipe");
		let recipe_ingredients = json_ld_store.get_recipe_ingredient_property(recipe_id);
		assert_eq!(recipe_ingredients.len(), 9);
		let recipe_instructions = json_ld_store.get_recipe_instructions_property(recipe_id);
		for recipe_instruction in recipe_instructions {
			let recipe_instruction_type = json_ld_store
				.get_type(recipe_instruction.as_id().unwrap())
				.unwrap();
			assert_eq!(
				recipe_instruction_type.as_id().unwrap().as_str(),
				HOW_TO_SECTION_IRI_HTTP
			);
		}
		let nutrition_information_ids = json_ld_store.find_nutrition_information_ids();
		let nutrition_information_id = *nutrition_information_ids.first().unwrap();
		let calories = json_ld_store.get_calories_property(nutrition_information_id);
		assert_eq!(
			(*calories.first().unwrap()).as_literal().unwrap().as_str(),
			"420 kcal"
		);
	}
}
