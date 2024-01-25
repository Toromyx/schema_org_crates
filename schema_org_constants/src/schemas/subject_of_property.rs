/// <https://schema.org/subjectOf>
pub const SUBJECT_OF_PROPERTY_IRI_HTTP: &str = "http://schema.org/subjectOf";
/// <https://schema.org/subjectOf>
pub const SUBJECT_OF_PROPERTY_IRI_HTTPS: &str = "https://schema.org/subjectOf";
/// <https://schema.org/subjectOf>
pub const SUBJECT_OF_PROPERTY_LABEL: &str = "subjectOf";
pub struct SubjectOfPropertyIri;
impl PartialEq<&str> for SubjectOfPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUBJECT_OF_PROPERTY_IRI_HTTP || *other == SUBJECT_OF_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SubjectOfPropertyIri> for &str {
	fn eq(&self, other: &SubjectOfPropertyIri) -> bool {
		*self == SUBJECT_OF_PROPERTY_IRI_HTTP || *self == SUBJECT_OF_PROPERTY_IRI_HTTPS
	}
}
pub struct SubjectOfPropertyIriOrLabel;
impl PartialEq<&str> for SubjectOfPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SubjectOfPropertyIri || *other == SUBJECT_OF_PROPERTY_LABEL
	}
}
impl PartialEq<SubjectOfPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SubjectOfPropertyIriOrLabel) -> bool {
		*self == SubjectOfPropertyIri || *self == SUBJECT_OF_PROPERTY_LABEL
	}
}
