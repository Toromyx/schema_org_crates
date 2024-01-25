/// <https://schema.org/VisualArtwork>
pub const VISUAL_ARTWORK_IRI_HTTP: &str = "http://schema.org/VisualArtwork";
/// <https://schema.org/VisualArtwork>
pub const VISUAL_ARTWORK_IRI_HTTPS: &str = "https://schema.org/VisualArtwork";
/// <https://schema.org/VisualArtwork>
pub const VISUAL_ARTWORK_LABEL: &str = "VisualArtwork";
pub struct VisualArtworkIri;
impl PartialEq<&str> for VisualArtworkIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VISUAL_ARTWORK_IRI_HTTP || *other == VISUAL_ARTWORK_IRI_HTTPS
	}
}
impl PartialEq<VisualArtworkIri> for &str {
	fn eq(&self, other: &VisualArtworkIri) -> bool {
		*self == VISUAL_ARTWORK_IRI_HTTP || *self == VISUAL_ARTWORK_IRI_HTTPS
	}
}
pub struct VisualArtworkIriOrLabel;
impl PartialEq<&str> for VisualArtworkIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VisualArtworkIri || *other == VISUAL_ARTWORK_LABEL
	}
}
impl PartialEq<VisualArtworkIriOrLabel> for &str {
	fn eq(&self, other: &VisualArtworkIriOrLabel) -> bool {
		*self == VisualArtworkIri || *self == VISUAL_ARTWORK_LABEL
	}
}
