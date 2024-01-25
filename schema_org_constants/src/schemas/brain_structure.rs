/// <https://schema.org/BrainStructure>
pub const BRAIN_STRUCTURE_IRI_HTTP: &str = "http://schema.org/BrainStructure";
/// <https://schema.org/BrainStructure>
pub const BRAIN_STRUCTURE_IRI_HTTPS: &str = "https://schema.org/BrainStructure";
/// <https://schema.org/BrainStructure>
pub const BRAIN_STRUCTURE_LABEL: &str = "BrainStructure";
pub struct BrainStructureIri;
impl PartialEq<&str> for BrainStructureIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BRAIN_STRUCTURE_IRI_HTTP || *other == BRAIN_STRUCTURE_IRI_HTTPS
	}
}
impl PartialEq<BrainStructureIri> for &str {
	fn eq(&self, other: &BrainStructureIri) -> bool {
		*self == BRAIN_STRUCTURE_IRI_HTTP || *self == BRAIN_STRUCTURE_IRI_HTTPS
	}
}
pub struct BrainStructureIriOrLabel;
impl PartialEq<&str> for BrainStructureIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BrainStructureIri || *other == BRAIN_STRUCTURE_LABEL
	}
}
impl PartialEq<BrainStructureIriOrLabel> for &str {
	fn eq(&self, other: &BrainStructureIriOrLabel) -> bool {
		*self == BrainStructureIri || *self == BRAIN_STRUCTURE_LABEL
	}
}
