/// <https://schema.org/educationRequirements>
pub const EDUCATION_REQUIREMENTS_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/educationRequirements";
/// <https://schema.org/educationRequirements>
pub const EDUCATION_REQUIREMENTS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/educationRequirements";
/// <https://schema.org/educationRequirements>
pub const EDUCATION_REQUIREMENTS_PROPERTY_LABEL: &str = "educationRequirements";
pub struct EducationRequirementsPropertyIri;
impl PartialEq<&str> for EducationRequirementsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EDUCATION_REQUIREMENTS_PROPERTY_IRI_HTTP
			|| *other == EDUCATION_REQUIREMENTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EducationRequirementsPropertyIri> for &str {
	fn eq(&self, other: &EducationRequirementsPropertyIri) -> bool {
		*self == EDUCATION_REQUIREMENTS_PROPERTY_IRI_HTTP
			|| *self == EDUCATION_REQUIREMENTS_PROPERTY_IRI_HTTPS
	}
}
pub struct EducationRequirementsPropertyIriOrLabel;
impl PartialEq<&str> for EducationRequirementsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EducationRequirementsPropertyIri
			|| *other == EDUCATION_REQUIREMENTS_PROPERTY_LABEL
	}
}
impl PartialEq<EducationRequirementsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EducationRequirementsPropertyIriOrLabel) -> bool {
		*self == EducationRequirementsPropertyIri || *self == EDUCATION_REQUIREMENTS_PROPERTY_LABEL
	}
}
