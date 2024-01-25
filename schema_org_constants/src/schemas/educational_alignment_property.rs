/// <https://schema.org/educationalAlignment>
pub const EDUCATIONAL_ALIGNMENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/educationalAlignment";
/// <https://schema.org/educationalAlignment>
pub const EDUCATIONAL_ALIGNMENT_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/educationalAlignment";
/// <https://schema.org/educationalAlignment>
pub const EDUCATIONAL_ALIGNMENT_PROPERTY_LABEL: &str = "educationalAlignment";
pub struct EducationalAlignmentPropertyIri;
impl PartialEq<&str> for EducationalAlignmentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EDUCATIONAL_ALIGNMENT_PROPERTY_IRI_HTTP
			|| *other == EDUCATIONAL_ALIGNMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EducationalAlignmentPropertyIri> for &str {
	fn eq(&self, other: &EducationalAlignmentPropertyIri) -> bool {
		*self == EDUCATIONAL_ALIGNMENT_PROPERTY_IRI_HTTP
			|| *self == EDUCATIONAL_ALIGNMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct EducationalAlignmentPropertyIriOrLabel;
impl PartialEq<&str> for EducationalAlignmentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EducationalAlignmentPropertyIri || *other == EDUCATIONAL_ALIGNMENT_PROPERTY_LABEL
	}
}
impl PartialEq<EducationalAlignmentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EducationalAlignmentPropertyIriOrLabel) -> bool {
		*self == EducationalAlignmentPropertyIri || *self == EDUCATIONAL_ALIGNMENT_PROPERTY_LABEL
	}
}
