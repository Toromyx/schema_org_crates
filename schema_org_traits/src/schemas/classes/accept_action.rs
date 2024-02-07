/// <https://schema.org/AcceptAction>
pub trait FindAcceptActionIds {
	type IdType;
	/// <https://schema.org/AcceptAction>
	fn find_accept_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindAcceptActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_accept_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ACCEPT_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ACCEPT_ACTION_IRI_HTTPS,
			})
		}
	}
}
