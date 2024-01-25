/// <https://schema.org/ExercisePlan>
pub trait FindExercisePlanIds {
	type IdType;
	fn find_exercise_plan_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindExercisePlanIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_exercise_plan_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::EXERCISE_PLAN_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::EXERCISE_PLAN_IRI_HTTPS,
			})
		}
	}
}
