/// <https://schema.org/ComicCoverArt>
pub const COMIC_COVER_ART_IRI_HTTP: &str = "http://schema.org/ComicCoverArt";
/// <https://schema.org/ComicCoverArt>
pub const COMIC_COVER_ART_IRI_HTTPS: &str = "https://schema.org/ComicCoverArt";
/// <https://schema.org/ComicCoverArt>
pub const COMIC_COVER_ART_LABEL: &str = "ComicCoverArt";
pub struct ComicCoverArtIri;
impl PartialEq<&str> for ComicCoverArtIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMIC_COVER_ART_IRI_HTTP || *other == COMIC_COVER_ART_IRI_HTTPS
	}
}
impl PartialEq<ComicCoverArtIri> for &str {
	fn eq(&self, other: &ComicCoverArtIri) -> bool {
		*self == COMIC_COVER_ART_IRI_HTTP || *self == COMIC_COVER_ART_IRI_HTTPS
	}
}
pub struct ComicCoverArtIriOrLabel;
impl PartialEq<&str> for ComicCoverArtIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ComicCoverArtIri || *other == COMIC_COVER_ART_LABEL
	}
}
impl PartialEq<ComicCoverArtIriOrLabel> for &str {
	fn eq(&self, other: &ComicCoverArtIriOrLabel) -> bool {
		*self == ComicCoverArtIri || *self == COMIC_COVER_ART_LABEL
	}
}
