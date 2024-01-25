/// <https://schema.org/CoverArt>
pub const COVER_ART_IRI_HTTP: &str = "http://schema.org/CoverArt";
/// <https://schema.org/CoverArt>
pub const COVER_ART_IRI_HTTPS: &str = "https://schema.org/CoverArt";
/// <https://schema.org/CoverArt>
pub const COVER_ART_LABEL: &str = "CoverArt";
pub struct CoverArtIri;
impl PartialEq<&str> for CoverArtIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COVER_ART_IRI_HTTP || *other == COVER_ART_IRI_HTTPS
	}
}
impl PartialEq<CoverArtIri> for &str {
	fn eq(&self, other: &CoverArtIri) -> bool {
		*self == COVER_ART_IRI_HTTP || *self == COVER_ART_IRI_HTTPS
	}
}
pub struct CoverArtIriOrLabel;
impl PartialEq<&str> for CoverArtIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CoverArtIri || *other == COVER_ART_LABEL
	}
}
impl PartialEq<CoverArtIriOrLabel> for &str {
	fn eq(&self, other: &CoverArtIriOrLabel) -> bool {
		*self == CoverArtIri || *self == COVER_ART_LABEL
	}
}
