/// <https://schema.org/VideoGallery>
pub const VIDEO_GALLERY_IRI_HTTP: &str = "http://schema.org/VideoGallery";
/// <https://schema.org/VideoGallery>
pub const VIDEO_GALLERY_IRI_HTTPS: &str = "https://schema.org/VideoGallery";
/// <https://schema.org/VideoGallery>
pub const VIDEO_GALLERY_LABEL: &str = "VideoGallery";
pub struct VideoGalleryIri;
impl PartialEq<&str> for VideoGalleryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VIDEO_GALLERY_IRI_HTTP || *other == VIDEO_GALLERY_IRI_HTTPS
	}
}
impl PartialEq<VideoGalleryIri> for &str {
	fn eq(&self, other: &VideoGalleryIri) -> bool {
		*self == VIDEO_GALLERY_IRI_HTTP || *self == VIDEO_GALLERY_IRI_HTTPS
	}
}
pub struct VideoGalleryIriOrLabel;
impl PartialEq<&str> for VideoGalleryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VideoGalleryIri || *other == VIDEO_GALLERY_LABEL
	}
}
impl PartialEq<VideoGalleryIriOrLabel> for &str {
	fn eq(&self, other: &VideoGalleryIriOrLabel) -> bool {
		*self == VideoGalleryIri || *self == VIDEO_GALLERY_LABEL
	}
}
