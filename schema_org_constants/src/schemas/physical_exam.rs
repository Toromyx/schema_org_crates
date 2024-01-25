/// <https://schema.org/PhysicalExam>
pub const PHYSICAL_EXAM_IRI_HTTP: &str = "http://schema.org/PhysicalExam";
/// <https://schema.org/PhysicalExam>
pub const PHYSICAL_EXAM_IRI_HTTPS: &str = "https://schema.org/PhysicalExam";
/// <https://schema.org/PhysicalExam>
pub const PHYSICAL_EXAM_LABEL: &str = "PhysicalExam";
pub struct PhysicalExamIri;
impl PartialEq<&str> for PhysicalExamIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PHYSICAL_EXAM_IRI_HTTP || *other == PHYSICAL_EXAM_IRI_HTTPS
	}
}
impl PartialEq<PhysicalExamIri> for &str {
	fn eq(&self, other: &PhysicalExamIri) -> bool {
		*self == PHYSICAL_EXAM_IRI_HTTP || *self == PHYSICAL_EXAM_IRI_HTTPS
	}
}
pub struct PhysicalExamIriOrLabel;
impl PartialEq<&str> for PhysicalExamIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PhysicalExamIri || *other == PHYSICAL_EXAM_LABEL
	}
}
impl PartialEq<PhysicalExamIriOrLabel> for &str {
	fn eq(&self, other: &PhysicalExamIriOrLabel) -> bool {
		*self == PhysicalExamIri || *self == PHYSICAL_EXAM_LABEL
	}
}
