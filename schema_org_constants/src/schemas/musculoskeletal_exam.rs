/// <https://schema.org/MusculoskeletalExam>
pub const MUSCULOSKELETAL_EXAM_IRI_HTTP: &str = "http://schema.org/MusculoskeletalExam";
/// <https://schema.org/MusculoskeletalExam>
pub const MUSCULOSKELETAL_EXAM_IRI_HTTPS: &str = "https://schema.org/MusculoskeletalExam";
/// <https://schema.org/MusculoskeletalExam>
pub const MUSCULOSKELETAL_EXAM_LABEL: &str = "MusculoskeletalExam";
pub struct MusculoskeletalExamIri;
impl PartialEq<&str> for MusculoskeletalExamIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSCULOSKELETAL_EXAM_IRI_HTTP || *other == MUSCULOSKELETAL_EXAM_IRI_HTTPS
	}
}
impl PartialEq<MusculoskeletalExamIri> for &str {
	fn eq(&self, other: &MusculoskeletalExamIri) -> bool {
		*self == MUSCULOSKELETAL_EXAM_IRI_HTTP || *self == MUSCULOSKELETAL_EXAM_IRI_HTTPS
	}
}
pub struct MusculoskeletalExamIriOrLabel;
impl PartialEq<&str> for MusculoskeletalExamIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MusculoskeletalExamIri || *other == MUSCULOSKELETAL_EXAM_LABEL
	}
}
impl PartialEq<MusculoskeletalExamIriOrLabel> for &str {
	fn eq(&self, other: &MusculoskeletalExamIriOrLabel) -> bool {
		*self == MusculoskeletalExamIri || *self == MUSCULOSKELETAL_EXAM_LABEL
	}
}
