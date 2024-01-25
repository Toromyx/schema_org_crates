/// <https://schema.org/OrganizeAction>
pub trait FindOrganizeActionIds {
	type IdType;
	fn find_organize_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOrganizeActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_organize_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::ORGANIZE_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::ORGANIZE_ACTION_IRI_HTTPS,
			})
		}
	}
}
