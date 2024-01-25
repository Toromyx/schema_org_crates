/// <https://schema.org/CheckInAction>
pub trait FindCheckInActionIds {
	type IdType;
	fn find_check_in_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindCheckInActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_check_in_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::CHECK_IN_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::CHECK_IN_ACTION_IRI_HTTPS,
			})
		}
	}
}
