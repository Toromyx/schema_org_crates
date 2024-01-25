/// <https://schema.org/EmployeeRole>
pub const EMPLOYEE_ROLE_IRI_HTTP: &str = "http://schema.org/EmployeeRole";
/// <https://schema.org/EmployeeRole>
pub const EMPLOYEE_ROLE_IRI_HTTPS: &str = "https://schema.org/EmployeeRole";
/// <https://schema.org/EmployeeRole>
pub const EMPLOYEE_ROLE_LABEL: &str = "EmployeeRole";
pub struct EmployeeRoleIri;
impl PartialEq<&str> for EmployeeRoleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EMPLOYEE_ROLE_IRI_HTTP || *other == EMPLOYEE_ROLE_IRI_HTTPS
	}
}
impl PartialEq<EmployeeRoleIri> for &str {
	fn eq(&self, other: &EmployeeRoleIri) -> bool {
		*self == EMPLOYEE_ROLE_IRI_HTTP || *self == EMPLOYEE_ROLE_IRI_HTTPS
	}
}
pub struct EmployeeRoleIriOrLabel;
impl PartialEq<&str> for EmployeeRoleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EmployeeRoleIri || *other == EMPLOYEE_ROLE_LABEL
	}
}
impl PartialEq<EmployeeRoleIriOrLabel> for &str {
	fn eq(&self, other: &EmployeeRoleIriOrLabel) -> bool {
		*self == EmployeeRoleIri || *self == EMPLOYEE_ROLE_LABEL
	}
}
