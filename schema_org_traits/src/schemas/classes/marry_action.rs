/// <https://schema.org/MarryAction>
pub trait FindMarryActionIds {
	type IdType;
	/// <https://schema.org/MarryAction>
	fn find_marry_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindMarryActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_marry_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::MARRY_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::MARRY_ACTION_IRI_HTTPS,
			})
		}
	}
}
