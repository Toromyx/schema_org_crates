/// <https://schema.org/employmentType>
pub const EMPLOYMENT_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/employmentType";
/// <https://schema.org/employmentType>
pub const EMPLOYMENT_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/employmentType";
/// <https://schema.org/employmentType>
pub const EMPLOYMENT_TYPE_PROPERTY_LABEL: &str = "employmentType";
pub struct EmploymentTypePropertyIri;
impl PartialEq<&str> for EmploymentTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EMPLOYMENT_TYPE_PROPERTY_IRI_HTTP || *other == EMPLOYMENT_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EmploymentTypePropertyIri> for &str {
	fn eq(&self, other: &EmploymentTypePropertyIri) -> bool {
		*self == EMPLOYMENT_TYPE_PROPERTY_IRI_HTTP || *self == EMPLOYMENT_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct EmploymentTypePropertyIriOrLabel;
impl PartialEq<&str> for EmploymentTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EmploymentTypePropertyIri || *other == EMPLOYMENT_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<EmploymentTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &EmploymentTypePropertyIriOrLabel) -> bool {
		*self == EmploymentTypePropertyIri || *self == EMPLOYMENT_TYPE_PROPERTY_LABEL
	}
}
