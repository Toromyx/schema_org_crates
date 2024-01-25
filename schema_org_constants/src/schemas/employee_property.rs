/// <https://schema.org/employee>
pub const EMPLOYEE_PROPERTY_IRI_HTTP: &str = "http://schema.org/employee";
/// <https://schema.org/employee>
pub const EMPLOYEE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/employee";
/// <https://schema.org/employee>
pub const EMPLOYEE_PROPERTY_LABEL: &str = "employee";
pub struct EmployeePropertyIri;
impl PartialEq<&str> for EmployeePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EMPLOYEE_PROPERTY_IRI_HTTP || *other == EMPLOYEE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EmployeePropertyIri> for &str {
	fn eq(&self, other: &EmployeePropertyIri) -> bool {
		*self == EMPLOYEE_PROPERTY_IRI_HTTP || *self == EMPLOYEE_PROPERTY_IRI_HTTPS
	}
}
pub struct EmployeePropertyIriOrLabel;
impl PartialEq<&str> for EmployeePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EmployeePropertyIri || *other == EMPLOYEE_PROPERTY_LABEL
	}
}
impl PartialEq<EmployeePropertyIriOrLabel> for &str {
	fn eq(&self, other: &EmployeePropertyIriOrLabel) -> bool {
		*self == EmployeePropertyIri || *self == EMPLOYEE_PROPERTY_LABEL
	}
}
