/// <https://schema.org/OwnershipInfo>
pub trait FindOwnershipInfoIds {
	type IdType;
	/// <https://schema.org/OwnershipInfo>
	fn find_ownership_info_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindOwnershipInfoIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_ownership_info_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::OWNERSHIP_INFO_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::OWNERSHIP_INFO_IRI_HTTPS,
			})
		}
	}
}
