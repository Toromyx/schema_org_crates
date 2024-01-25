/// <https://schema.org/UseAction>
pub trait FindUseActionIds {
	type IdType;
	fn find_use_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindUseActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_use_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::USE_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::USE_ACTION_IRI_HTTPS,
			})
		}
	}
}
