/// <https://schema.org/proteinContent>
pub const PROTEIN_CONTENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/proteinContent";
/// <https://schema.org/proteinContent>
pub const PROTEIN_CONTENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/proteinContent";
/// <https://schema.org/proteinContent>
pub const PROTEIN_CONTENT_PROPERTY_LABEL: &str = "proteinContent";
pub struct ProteinContentPropertyIri;
impl PartialEq<&str> for ProteinContentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROTEIN_CONTENT_PROPERTY_IRI_HTTP || *other == PROTEIN_CONTENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProteinContentPropertyIri> for &str {
	fn eq(&self, other: &ProteinContentPropertyIri) -> bool {
		*self == PROTEIN_CONTENT_PROPERTY_IRI_HTTP || *self == PROTEIN_CONTENT_PROPERTY_IRI_HTTPS
	}
}
pub struct ProteinContentPropertyIriOrLabel;
impl PartialEq<&str> for ProteinContentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProteinContentPropertyIri || *other == PROTEIN_CONTENT_PROPERTY_LABEL
	}
}
impl PartialEq<ProteinContentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProteinContentPropertyIriOrLabel) -> bool {
		*self == ProteinContentPropertyIri || *self == PROTEIN_CONTENT_PROPERTY_LABEL
	}
}
