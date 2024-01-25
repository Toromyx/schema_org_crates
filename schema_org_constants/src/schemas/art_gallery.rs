/// <https://schema.org/ArtGallery>
pub const ART_GALLERY_IRI_HTTP: &str = "http://schema.org/ArtGallery";
/// <https://schema.org/ArtGallery>
pub const ART_GALLERY_IRI_HTTPS: &str = "https://schema.org/ArtGallery";
/// <https://schema.org/ArtGallery>
pub const ART_GALLERY_LABEL: &str = "ArtGallery";
pub struct ArtGalleryIri;
impl PartialEq<&str> for ArtGalleryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ART_GALLERY_IRI_HTTP || *other == ART_GALLERY_IRI_HTTPS
	}
}
impl PartialEq<ArtGalleryIri> for &str {
	fn eq(&self, other: &ArtGalleryIri) -> bool {
		*self == ART_GALLERY_IRI_HTTP || *self == ART_GALLERY_IRI_HTTPS
	}
}
pub struct ArtGalleryIriOrLabel;
impl PartialEq<&str> for ArtGalleryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArtGalleryIri || *other == ART_GALLERY_LABEL
	}
}
impl PartialEq<ArtGalleryIriOrLabel> for &str {
	fn eq(&self, other: &ArtGalleryIriOrLabel) -> bool {
		*self == ArtGalleryIri || *self == ART_GALLERY_LABEL
	}
}
