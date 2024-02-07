/// <https://schema.org/FDAcategoryB>
pub trait FindFdAcategoryBIds {
	type IdType;
	/// <https://schema.org/FDAcategoryB>
	fn find_fd_acategory_b_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindFdAcategoryBIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_fd_acategory_b_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::FD_ACATEGORY_B_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::FD_ACATEGORY_B_IRI_HTTPS,
			})
		}
	}
}
