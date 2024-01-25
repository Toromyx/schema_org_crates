/// <https://schema.org/ImageGallery>
pub const IMAGE_GALLERY_IRI_HTTP: &str = "http://schema.org/ImageGallery";
/// <https://schema.org/ImageGallery>
pub const IMAGE_GALLERY_IRI_HTTPS: &str = "https://schema.org/ImageGallery";
/// <https://schema.org/ImageGallery>
pub const IMAGE_GALLERY_LABEL: &str = "ImageGallery";
pub struct ImageGalleryIri;
impl PartialEq<&str> for ImageGalleryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IMAGE_GALLERY_IRI_HTTP || *other == IMAGE_GALLERY_IRI_HTTPS
	}
}
impl PartialEq<ImageGalleryIri> for &str {
	fn eq(&self, other: &ImageGalleryIri) -> bool {
		*self == IMAGE_GALLERY_IRI_HTTP || *self == IMAGE_GALLERY_IRI_HTTPS
	}
}
pub struct ImageGalleryIriOrLabel;
impl PartialEq<&str> for ImageGalleryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ImageGalleryIri || *other == IMAGE_GALLERY_LABEL
	}
}
impl PartialEq<ImageGalleryIriOrLabel> for &str {
	fn eq(&self, other: &ImageGalleryIriOrLabel) -> bool {
		*self == ImageGalleryIri || *self == IMAGE_GALLERY_LABEL
	}
}
