/// <https://schema.org/InstallAction>
pub trait FindInstallActionIds {
	type IdType;
	fn find_install_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindInstallActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_install_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::INSTALL_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::INSTALL_ACTION_IRI_HTTPS,
			})
		}
	}
}
