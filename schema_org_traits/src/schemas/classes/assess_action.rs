/// <https://schema.org/AssessAction>
pub trait FindAssessActionIds {
	type IdType;
	/// <https://schema.org/AssessAction>
	fn find_assess_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindAssessActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_assess_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ASSESS_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ASSESS_ACTION_IRI_HTTPS,
			})
		}
	}
}
