/// <https://schema.org/WarrantyScope>
pub trait FindWarrantyScopeIds {
	type IdType;
	/// <https://schema.org/WarrantyScope>
	fn find_warranty_scope_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindWarrantyScopeIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_warranty_scope_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::WARRANTY_SCOPE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::WARRANTY_SCOPE_IRI_HTTPS,
			})
		}
	}
}
