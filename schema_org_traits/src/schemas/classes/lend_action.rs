/// <https://schema.org/LendAction>
pub trait FindLendActionIds {
	type IdType;
	/// <https://schema.org/LendAction>
	fn find_lend_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindLendActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_lend_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::LEND_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::LEND_ACTION_IRI_HTTPS,
			})
		}
	}
}
