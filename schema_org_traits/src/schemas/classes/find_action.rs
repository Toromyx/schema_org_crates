/// <https://schema.org/FindAction>
pub trait FindFindActionIds {
	type IdType;
	/// <https://schema.org/FindAction>
	fn find_find_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindFindActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_find_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::FIND_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::FIND_ACTION_IRI_HTTPS,
			})
		}
	}
}
