/// <https://schema.org/educationalProgramMode>
pub const EDUCATIONAL_PROGRAM_MODE_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/educationalProgramMode";
/// <https://schema.org/educationalProgramMode>
pub const EDUCATIONAL_PROGRAM_MODE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/educationalProgramMode";
/// <https://schema.org/educationalProgramMode>
pub const EDUCATIONAL_PROGRAM_MODE_PROPERTY_LABEL: &str = "educationalProgramMode";
pub struct EducationalProgramModePropertyIri;
impl PartialEq<&str> for EducationalProgramModePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EDUCATIONAL_PROGRAM_MODE_PROPERTY_IRI_HTTP
			|| *other == EDUCATIONAL_PROGRAM_MODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EducationalProgramModePropertyIri> for &str {
	fn eq(&self, other: &EducationalProgramModePropertyIri) -> bool {
		*self == EDUCATIONAL_PROGRAM_MODE_PROPERTY_IRI_HTTP
			|| *self == EDUCATIONAL_PROGRAM_MODE_PROPERTY_IRI_HTTPS
	}
}
pub struct EducationalProgramModePropertyIriOrLabel;
impl PartialEq<&str> for EducationalProgramModePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EducationalProgramModePropertyIri
			|| *other == EDUCATIONAL_PROGRAM_MODE_PROPERTY_LABEL
	}
}
impl PartialEq<EducationalProgramModePropertyIriOrLabel> for &str {
	fn eq(&self, other: &EducationalProgramModePropertyIriOrLabel) -> bool {
		*self == EducationalProgramModePropertyIri
			|| *self == EDUCATIONAL_PROGRAM_MODE_PROPERTY_LABEL
	}
}
