/// <https://schema.org/CivicStructure>
pub const CIVIC_STRUCTURE_IRI_HTTP: &str = "http://schema.org/CivicStructure";
/// <https://schema.org/CivicStructure>
pub const CIVIC_STRUCTURE_IRI_HTTPS: &str = "https://schema.org/CivicStructure";
/// <https://schema.org/CivicStructure>
pub const CIVIC_STRUCTURE_LABEL: &str = "CivicStructure";
pub struct CivicStructureIri;
impl PartialEq<&str> for CivicStructureIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CIVIC_STRUCTURE_IRI_HTTP || *other == CIVIC_STRUCTURE_IRI_HTTPS
	}
}
impl PartialEq<CivicStructureIri> for &str {
	fn eq(&self, other: &CivicStructureIri) -> bool {
		*self == CIVIC_STRUCTURE_IRI_HTTP || *self == CIVIC_STRUCTURE_IRI_HTTPS
	}
}
pub struct CivicStructureIriOrLabel;
impl PartialEq<&str> for CivicStructureIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CivicStructureIri || *other == CIVIC_STRUCTURE_LABEL
	}
}
impl PartialEq<CivicStructureIriOrLabel> for &str {
	fn eq(&self, other: &CivicStructureIriOrLabel) -> bool {
		*self == CivicStructureIri || *self == CIVIC_STRUCTURE_LABEL
	}
}
