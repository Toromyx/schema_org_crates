/// <https://schema.org/numberOfEmployees>
pub const NUMBER_OF_EMPLOYEES_PROPERTY_IRI_HTTP: &str = "http://schema.org/numberOfEmployees";
/// <https://schema.org/numberOfEmployees>
pub const NUMBER_OF_EMPLOYEES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/numberOfEmployees";
/// <https://schema.org/numberOfEmployees>
pub const NUMBER_OF_EMPLOYEES_PROPERTY_LABEL: &str = "numberOfEmployees";
pub struct NumberOfEmployeesPropertyIri;
impl PartialEq<&str> for NumberOfEmployeesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUMBER_OF_EMPLOYEES_PROPERTY_IRI_HTTP
			|| *other == NUMBER_OF_EMPLOYEES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumberOfEmployeesPropertyIri> for &str {
	fn eq(&self, other: &NumberOfEmployeesPropertyIri) -> bool {
		*self == NUMBER_OF_EMPLOYEES_PROPERTY_IRI_HTTP
			|| *self == NUMBER_OF_EMPLOYEES_PROPERTY_IRI_HTTPS
	}
}
pub struct NumberOfEmployeesPropertyIriOrLabel;
impl PartialEq<&str> for NumberOfEmployeesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumberOfEmployeesPropertyIri || *other == NUMBER_OF_EMPLOYEES_PROPERTY_LABEL
	}
}
impl PartialEq<NumberOfEmployeesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumberOfEmployeesPropertyIriOrLabel) -> bool {
		*self == NumberOfEmployeesPropertyIri || *self == NUMBER_OF_EMPLOYEES_PROPERTY_LABEL
	}
}
