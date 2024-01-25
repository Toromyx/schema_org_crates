/// <https://schema.org/Sculpture>
pub const SCULPTURE_IRI_HTTP: &str = "http://schema.org/Sculpture";
/// <https://schema.org/Sculpture>
pub const SCULPTURE_IRI_HTTPS: &str = "https://schema.org/Sculpture";
/// <https://schema.org/Sculpture>
pub const SCULPTURE_LABEL: &str = "Sculpture";
pub struct SculptureIri;
impl PartialEq<&str> for SculptureIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SCULPTURE_IRI_HTTP || *other == SCULPTURE_IRI_HTTPS
	}
}
impl PartialEq<SculptureIri> for &str {
	fn eq(&self, other: &SculptureIri) -> bool {
		*self == SCULPTURE_IRI_HTTP || *self == SCULPTURE_IRI_HTTPS
	}
}
pub struct SculptureIriOrLabel;
impl PartialEq<&str> for SculptureIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SculptureIri || *other == SCULPTURE_LABEL
	}
}
impl PartialEq<SculptureIriOrLabel> for &str {
	fn eq(&self, other: &SculptureIriOrLabel) -> bool {
		*self == SculptureIri || *self == SCULPTURE_LABEL
	}
}
