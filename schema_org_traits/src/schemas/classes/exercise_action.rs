/// <https://schema.org/ExerciseAction>
pub trait FindExerciseActionIds {
	type IdType;
	/// <https://schema.org/ExerciseAction>
	fn find_exercise_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindExerciseActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_exercise_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::EXERCISE_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::EXERCISE_ACTION_IRI_HTTPS,
			})
		}
	}
}
