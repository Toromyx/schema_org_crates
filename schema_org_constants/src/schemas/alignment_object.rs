/// <https://schema.org/AlignmentObject>
pub const ALIGNMENT_OBJECT_IRI_HTTP: &str = "http://schema.org/AlignmentObject";
/// <https://schema.org/AlignmentObject>
pub const ALIGNMENT_OBJECT_IRI_HTTPS: &str = "https://schema.org/AlignmentObject";
/// <https://schema.org/AlignmentObject>
pub const ALIGNMENT_OBJECT_LABEL: &str = "AlignmentObject";
pub struct AlignmentObjectIri;
impl PartialEq<&str> for AlignmentObjectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ALIGNMENT_OBJECT_IRI_HTTP || *other == ALIGNMENT_OBJECT_IRI_HTTPS
	}
}
impl PartialEq<AlignmentObjectIri> for &str {
	fn eq(&self, other: &AlignmentObjectIri) -> bool {
		*self == ALIGNMENT_OBJECT_IRI_HTTP || *self == ALIGNMENT_OBJECT_IRI_HTTPS
	}
}
pub struct AlignmentObjectIriOrLabel;
impl PartialEq<&str> for AlignmentObjectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AlignmentObjectIri || *other == ALIGNMENT_OBJECT_LABEL
	}
}
impl PartialEq<AlignmentObjectIriOrLabel> for &str {
	fn eq(&self, other: &AlignmentObjectIriOrLabel) -> bool {
		*self == AlignmentObjectIri || *self == ALIGNMENT_OBJECT_LABEL
	}
}
