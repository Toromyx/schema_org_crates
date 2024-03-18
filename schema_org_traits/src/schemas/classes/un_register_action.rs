/// <https://schema.org/UnRegisterAction>
pub trait FindUnRegisterActionIds {
	type IdType;
	/// <https://schema.org/UnRegisterAction>
	fn find_un_register_action_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindUnRegisterActionIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_un_register_action_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::UN_REGISTER_ACTION_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::UN_REGISTER_ACTION_IRI_HTTPS,
			})
		}
	}
}