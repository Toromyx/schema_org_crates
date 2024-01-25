/// <https://schema.org/AnatomicalStructure>
pub const ANATOMICAL_STRUCTURE_IRI_HTTP: &str = "http://schema.org/AnatomicalStructure";
/// <https://schema.org/AnatomicalStructure>
pub const ANATOMICAL_STRUCTURE_IRI_HTTPS: &str = "https://schema.org/AnatomicalStructure";
/// <https://schema.org/AnatomicalStructure>
pub const ANATOMICAL_STRUCTURE_LABEL: &str = "AnatomicalStructure";
pub struct AnatomicalStructureIri;
impl PartialEq<&str> for AnatomicalStructureIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ANATOMICAL_STRUCTURE_IRI_HTTP || *other == ANATOMICAL_STRUCTURE_IRI_HTTPS
	}
}
impl PartialEq<AnatomicalStructureIri> for &str {
	fn eq(&self, other: &AnatomicalStructureIri) -> bool {
		*self == ANATOMICAL_STRUCTURE_IRI_HTTP || *self == ANATOMICAL_STRUCTURE_IRI_HTTPS
	}
}
pub struct AnatomicalStructureIriOrLabel;
impl PartialEq<&str> for AnatomicalStructureIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AnatomicalStructureIri || *other == ANATOMICAL_STRUCTURE_LABEL
	}
}
impl PartialEq<AnatomicalStructureIriOrLabel> for &str {
	fn eq(&self, other: &AnatomicalStructureIriOrLabel) -> bool {
		*self == AnatomicalStructureIri || *self == ANATOMICAL_STRUCTURE_LABEL
	}
}
