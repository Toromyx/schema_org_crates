/// <https://schema.org/educationalFramework>
pub const EDUCATIONAL_FRAMEWORK_PROPERTY_IRI_HTTP: &str = "http://schema.org/educationalFramework";
/// <https://schema.org/educationalFramework>
pub const EDUCATIONAL_FRAMEWORK_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/educationalFramework";
/// <https://schema.org/educationalFramework>
pub const EDUCATIONAL_FRAMEWORK_PROPERTY_LABEL: &str = "educationalFramework";
pub struct EducationalFrameworkPropertyIri;
impl PartialEq<&str> for EducationalFrameworkPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EDUCATIONAL_FRAMEWORK_PROPERTY_IRI_HTTP
			|| *other == EDUCATIONAL_FRAMEWORK_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EducationalFrameworkPropertyIri> for &str {
	fn eq(&self, other: &EducationalFrameworkPropertyIri) -> bool {
		*self == EDUCATIONAL_FRAMEWORK_PROPERTY_IRI_HTTP
			|| *self == EDUCATIONAL_FRAMEWORK_PROPERTY_IRI_HTTPS
	}
}
pub struct EducationalFrameworkPropertyIriOrLabel;
impl PartialEq<&str> for EducationalFrameworkPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EducationalFrameworkPropertyIri || *other == EDUCATIONAL_FRAMEWORK_PROPERTY_LABEL
	}
}
impl PartialEq<EducationalFrameworkPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EducationalFrameworkPropertyIriOrLabel) -> bool {
		*self == EducationalFrameworkPropertyIri || *self == EDUCATIONAL_FRAMEWORK_PROPERTY_LABEL
	}
}
