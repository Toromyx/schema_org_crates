/// <https://schema.org/EmployeeRole>
pub trait FindEmployeeRoleIds {
	type IdType;
	fn find_employee_role_ids(&self) -> Vec<&Self::IdType>;
}
#[cfg(any(feature = "json-ld_0_15", doc))]
mod json_ld_0_15 {
	use schema_org_constants::SchemaOrgNamespace;
	impl super::FindEmployeeRoleIds for crate::json_ld_0_15::JsonLdStore {
		type IdType = json_ld_0_15::ValidId;
		fn find_employee_role_ids(&self) -> Vec<&Self::IdType> {
			self.find_schema(match self.namespace() {
				SchemaOrgNamespace::Http => schema_org_constants::EMPLOYEE_ROLE_IRI_HTTP,
				SchemaOrgNamespace::Https => schema_org_constants::EMPLOYEE_ROLE_IRI_HTTPS,
			})
		}
	}
}
