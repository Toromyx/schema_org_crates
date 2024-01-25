/// <https://schema.org/NewCondition>
pub trait FindNewConditionIds {
	type IdType;
	fn find_new_condition_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindNewConditionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_new_condition_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::NEW_CONDITION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::NEW_CONDITION_IRI_HTTPS,
			})
		}
	}
}
