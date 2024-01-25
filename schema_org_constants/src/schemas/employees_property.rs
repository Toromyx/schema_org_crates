/// <https://schema.org/employees>
#[deprecated = "This schema is superseded by <https://schema.org/employee>."]
pub const EMPLOYEES_PROPERTY_IRI_HTTP: &str = "http://schema.org/employees";
/// <https://schema.org/employees>
#[deprecated = "This schema is superseded by <https://schema.org/employee>."]
pub const EMPLOYEES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/employees";
/// <https://schema.org/employees>
#[deprecated = "This schema is superseded by <https://schema.org/employee>."]
pub const EMPLOYEES_PROPERTY_LABEL: &str = "employees";
pub struct EmployeesPropertyIri;
impl PartialEq<&str> for EmployeesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EMPLOYEES_PROPERTY_IRI_HTTP || *other == EMPLOYEES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EmployeesPropertyIri> for &str {
	fn eq(&self, other: &EmployeesPropertyIri) -> bool {
		*self == EMPLOYEES_PROPERTY_IRI_HTTP || *self == EMPLOYEES_PROPERTY_IRI_HTTPS
	}
}
pub struct EmployeesPropertyIriOrLabel;
impl PartialEq<&str> for EmployeesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EmployeesPropertyIri || *other == EMPLOYEES_PROPERTY_LABEL
	}
}
impl PartialEq<EmployeesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EmployeesPropertyIriOrLabel) -> bool {
		*self == EmployeesPropertyIri || *self == EMPLOYEES_PROPERTY_LABEL
	}
}
