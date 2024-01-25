/// <https://schema.org/CardiovascularExam>
pub const CARDIOVASCULAR_EXAM_IRI_HTTP: &str = "http://schema.org/CardiovascularExam";
/// <https://schema.org/CardiovascularExam>
pub const CARDIOVASCULAR_EXAM_IRI_HTTPS: &str = "https://schema.org/CardiovascularExam";
/// <https://schema.org/CardiovascularExam>
pub const CARDIOVASCULAR_EXAM_LABEL: &str = "CardiovascularExam";
pub struct CardiovascularExamIri;
impl PartialEq<&str> for CardiovascularExamIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CARDIOVASCULAR_EXAM_IRI_HTTP || *other == CARDIOVASCULAR_EXAM_IRI_HTTPS
	}
}
impl PartialEq<CardiovascularExamIri> for &str {
	fn eq(&self, other: &CardiovascularExamIri) -> bool {
		*self == CARDIOVASCULAR_EXAM_IRI_HTTP || *self == CARDIOVASCULAR_EXAM_IRI_HTTPS
	}
}
pub struct CardiovascularExamIriOrLabel;
impl PartialEq<&str> for CardiovascularExamIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CardiovascularExamIri || *other == CARDIOVASCULAR_EXAM_LABEL
	}
}
impl PartialEq<CardiovascularExamIriOrLabel> for &str {
	fn eq(&self, other: &CardiovascularExamIriOrLabel) -> bool {
		*self == CardiovascularExamIri || *self == CARDIOVASCULAR_EXAM_LABEL
	}
}
