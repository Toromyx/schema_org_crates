/// <https://schema.org/relatedStructure>
pub const RELATED_STRUCTURE_PROPERTY_IRI_HTTP: &str = "http://schema.org/relatedStructure";
/// <https://schema.org/relatedStructure>
pub const RELATED_STRUCTURE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/relatedStructure";
/// <https://schema.org/relatedStructure>
pub const RELATED_STRUCTURE_PROPERTY_LABEL: &str = "relatedStructure";
pub struct RelatedStructurePropertyIri;
impl PartialEq<&str> for RelatedStructurePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RELATED_STRUCTURE_PROPERTY_IRI_HTTP
			|| *other == RELATED_STRUCTURE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RelatedStructurePropertyIri> for &str {
	fn eq(&self, other: &RelatedStructurePropertyIri) -> bool {
		*self == RELATED_STRUCTURE_PROPERTY_IRI_HTTP
			|| *self == RELATED_STRUCTURE_PROPERTY_IRI_HTTPS
	}
}
pub struct RelatedStructurePropertyIriOrLabel;
impl PartialEq<&str> for RelatedStructurePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RelatedStructurePropertyIri || *other == RELATED_STRUCTURE_PROPERTY_LABEL
	}
}
impl PartialEq<RelatedStructurePropertyIriOrLabel> for &str {
	fn eq(&self, other: &RelatedStructurePropertyIriOrLabel) -> bool {
		*self == RelatedStructurePropertyIri || *self == RELATED_STRUCTURE_PROPERTY_LABEL
	}
}
