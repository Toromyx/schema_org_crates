/// <https://schema.org/LinkRole>
pub trait FindLinkRoleIds {
	type IdType;
	/// <https://schema.org/LinkRole>
	fn find_link_role_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindLinkRoleIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_link_role_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::LINK_ROLE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::LINK_ROLE_IRI_HTTPS,
			})
		}
	}
}
