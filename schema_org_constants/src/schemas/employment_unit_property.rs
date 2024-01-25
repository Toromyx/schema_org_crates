/// <https://schema.org/employmentUnit>
pub const EMPLOYMENT_UNIT_PROPERTY_IRI_HTTP: &str = "http://schema.org/employmentUnit";
/// <https://schema.org/employmentUnit>
pub const EMPLOYMENT_UNIT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/employmentUnit";
/// <https://schema.org/employmentUnit>
pub const EMPLOYMENT_UNIT_PROPERTY_LABEL: &str = "employmentUnit";
pub struct EmploymentUnitPropertyIri;
impl PartialEq<&str> for EmploymentUnitPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EMPLOYMENT_UNIT_PROPERTY_IRI_HTTP || *other == EMPLOYMENT_UNIT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EmploymentUnitPropertyIri> for &str {
	fn eq(&self, other: &EmploymentUnitPropertyIri) -> bool {
		*self == EMPLOYMENT_UNIT_PROPERTY_IRI_HTTP || *self == EMPLOYMENT_UNIT_PROPERTY_IRI_HTTPS
	}
}
pub struct EmploymentUnitPropertyIriOrLabel;
impl PartialEq<&str> for EmploymentUnitPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EmploymentUnitPropertyIri || *other == EMPLOYMENT_UNIT_PROPERTY_LABEL
	}
}
impl PartialEq<EmploymentUnitPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EmploymentUnitPropertyIriOrLabel) -> bool {
		*self == EmploymentUnitPropertyIri || *self == EMPLOYMENT_UNIT_PROPERTY_LABEL
	}
}
