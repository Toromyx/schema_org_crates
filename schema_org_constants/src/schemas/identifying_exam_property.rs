/// <https://schema.org/identifyingExam>
pub const IDENTIFYING_EXAM_PROPERTY_IRI_HTTP: &str = "http://schema.org/identifyingExam";
/// <https://schema.org/identifyingExam>
pub const IDENTIFYING_EXAM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/identifyingExam";
/// <https://schema.org/identifyingExam>
pub const IDENTIFYING_EXAM_PROPERTY_LABEL: &str = "identifyingExam";
pub struct IdentifyingExamPropertyIri;
impl PartialEq<&str> for IdentifyingExamPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IDENTIFYING_EXAM_PROPERTY_IRI_HTTP
			|| *other == IDENTIFYING_EXAM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IdentifyingExamPropertyIri> for &str {
	fn eq(&self, other: &IdentifyingExamPropertyIri) -> bool {
		*self == IDENTIFYING_EXAM_PROPERTY_IRI_HTTP || *self == IDENTIFYING_EXAM_PROPERTY_IRI_HTTPS
	}
}
pub struct IdentifyingExamPropertyIriOrLabel;
impl PartialEq<&str> for IdentifyingExamPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IdentifyingExamPropertyIri || *other == IDENTIFYING_EXAM_PROPERTY_LABEL
	}
}
impl PartialEq<IdentifyingExamPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IdentifyingExamPropertyIriOrLabel) -> bool {
		*self == IdentifyingExamPropertyIri || *self == IDENTIFYING_EXAM_PROPERTY_LABEL
	}
}
