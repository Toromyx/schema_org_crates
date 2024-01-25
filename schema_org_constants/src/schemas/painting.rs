/// <https://schema.org/Painting>
pub const PAINTING_IRI_HTTP: &str = "http://schema.org/Painting";
/// <https://schema.org/Painting>
pub const PAINTING_IRI_HTTPS: &str = "https://schema.org/Painting";
/// <https://schema.org/Painting>
pub const PAINTING_LABEL: &str = "Painting";
pub struct PaintingIri;
impl PartialEq<&str> for PaintingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAINTING_IRI_HTTP || *other == PAINTING_IRI_HTTPS
	}
}
impl PartialEq<PaintingIri> for &str {
	fn eq(&self, other: &PaintingIri) -> bool {
		*self == PAINTING_IRI_HTTP || *self == PAINTING_IRI_HTTPS
	}
}
pub struct PaintingIriOrLabel;
impl PartialEq<&str> for PaintingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PaintingIri || *other == PAINTING_LABEL
	}
}
impl PartialEq<PaintingIriOrLabel> for &str {
	fn eq(&self, other: &PaintingIriOrLabel) -> bool {
		*self == PaintingIri || *self == PAINTING_LABEL
	}
}
