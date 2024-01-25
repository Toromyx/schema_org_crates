/// <https://schema.org/subStructure>
pub const SUB_STRUCTURE_PROPERTY_IRI_HTTP: &str = "http://schema.org/subStructure";
/// <https://schema.org/subStructure>
pub const SUB_STRUCTURE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/subStructure";
/// <https://schema.org/subStructure>
pub const SUB_STRUCTURE_PROPERTY_LABEL: &str = "subStructure";
pub struct SubStructurePropertyIri;
impl PartialEq<&str> for SubStructurePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUB_STRUCTURE_PROPERTY_IRI_HTTP || *other == SUB_STRUCTURE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SubStructurePropertyIri> for &str {
	fn eq(&self, other: &SubStructurePropertyIri) -> bool {
		*self == SUB_STRUCTURE_PROPERTY_IRI_HTTP || *self == SUB_STRUCTURE_PROPERTY_IRI_HTTPS
	}
}
pub struct SubStructurePropertyIriOrLabel;
impl PartialEq<&str> for SubStructurePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SubStructurePropertyIri || *other == SUB_STRUCTURE_PROPERTY_LABEL
	}
}
impl PartialEq<SubStructurePropertyIriOrLabel> for &str {
	fn eq(&self, other: &SubStructurePropertyIriOrLabel) -> bool {
		*self == SubStructurePropertyIri || *self == SUB_STRUCTURE_PROPERTY_LABEL
	}
}
