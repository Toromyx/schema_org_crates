/// <https://schema.org/DepartmentStore>
pub trait FindDepartmentStoreIds {
	type IdType;
	/// <https://schema.org/DepartmentStore>
	fn find_department_store_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindDepartmentStoreIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_department_store_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::DEPARTMENT_STORE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::DEPARTMENT_STORE_IRI_HTTPS,
			})
		}
	}
}
