/// <https://schema.org/alignmentType>
pub const ALIGNMENT_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/alignmentType";
/// <https://schema.org/alignmentType>
pub const ALIGNMENT_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/alignmentType";
/// <https://schema.org/alignmentType>
pub const ALIGNMENT_TYPE_PROPERTY_LABEL: &str = "alignmentType";
pub struct AlignmentTypePropertyIri;
impl PartialEq<&str> for AlignmentTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ALIGNMENT_TYPE_PROPERTY_IRI_HTTP || *other == ALIGNMENT_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AlignmentTypePropertyIri> for &str {
	fn eq(&self, other: &AlignmentTypePropertyIri) -> bool {
		*self == ALIGNMENT_TYPE_PROPERTY_IRI_HTTP || *self == ALIGNMENT_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct AlignmentTypePropertyIriOrLabel;
impl PartialEq<&str> for AlignmentTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AlignmentTypePropertyIri || *other == ALIGNMENT_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<AlignmentTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AlignmentTypePropertyIriOrLabel) -> bool {
		*self == AlignmentTypePropertyIri || *self == ALIGNMENT_TYPE_PROPERTY_LABEL
	}
}
