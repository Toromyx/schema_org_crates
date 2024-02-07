/// <https://schema.org/FailedActionStatus>
pub trait FindFailedActionStatusIds {
	type IdType;
	/// <https://schema.org/FailedActionStatus>
	fn find_failed_action_status_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindFailedActionStatusIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_failed_action_status_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::FAILED_ACTION_STATUS_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::FAILED_ACTION_STATUS_IRI_HTTPS,
			})
		}
	}
}
