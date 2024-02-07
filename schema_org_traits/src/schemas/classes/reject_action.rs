/// <https://schema.org/RejectAction>
pub trait FindRejectActionIds {
	type IdType;
	/// <https://schema.org/RejectAction>
	fn find_reject_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindRejectActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_reject_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::REJECT_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::REJECT_ACTION_IRI_HTTPS,
			})
		}
	}
}
