/// <https://schema.org/MediaGallery>
pub const MEDIA_GALLERY_IRI_HTTP: &str = "http://schema.org/MediaGallery";
/// <https://schema.org/MediaGallery>
pub const MEDIA_GALLERY_IRI_HTTPS: &str = "https://schema.org/MediaGallery";
/// <https://schema.org/MediaGallery>
pub const MEDIA_GALLERY_LABEL: &str = "MediaGallery";
pub struct MediaGalleryIri;
impl PartialEq<&str> for MediaGalleryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDIA_GALLERY_IRI_HTTP || *other == MEDIA_GALLERY_IRI_HTTPS
	}
}
impl PartialEq<MediaGalleryIri> for &str {
	fn eq(&self, other: &MediaGalleryIri) -> bool {
		*self == MEDIA_GALLERY_IRI_HTTP || *self == MEDIA_GALLERY_IRI_HTTPS
	}
}
pub struct MediaGalleryIriOrLabel;
impl PartialEq<&str> for MediaGalleryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MediaGalleryIri || *other == MEDIA_GALLERY_LABEL
	}
}
impl PartialEq<MediaGalleryIriOrLabel> for &str {
	fn eq(&self, other: &MediaGalleryIriOrLabel) -> bool {
		*self == MediaGalleryIri || *self == MEDIA_GALLERY_LABEL
	}
}
