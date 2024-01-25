/// <https://schema.org/FDAcategoryC>
pub trait FindFdAcategoryCIds {
	type IdType;
	fn find_fd_acategory_c_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindFdAcategoryCIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_fd_acategory_c_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::FD_ACATEGORY_C_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::FD_ACATEGORY_C_IRI_HTTPS,
			})
		}
	}
}
