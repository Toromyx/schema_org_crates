/// <https://schema.org/CompletedActionStatus>
pub trait FindCompletedActionStatusIds {
	type IdType;
	/// <https://schema.org/CompletedActionStatus>
	fn find_completed_action_status_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindCompletedActionStatusIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_completed_action_status_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::COMPLETED_ACTION_STATUS_IRI_HTTP,
				SchemaOrgNamespace::Https => {
					schema_org_constants::COMPLETED_ACTION_STATUS_IRI_HTTPS
				}
			})
		}
	}
}
