/// <https://schema.org/ActionStatusType>
pub trait FindActionStatusTypeIds {
	type IdType;
	/// <https://schema.org/ActionStatusType>
	fn find_action_status_type_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindActionStatusTypeIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_action_status_type_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ACTION_STATUS_TYPE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ACTION_STATUS_TYPE_IRI_HTTPS,
			})
		}
	}
}
