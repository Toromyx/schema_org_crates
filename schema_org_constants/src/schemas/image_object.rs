/// <https://schema.org/ImageObject>
pub const IMAGE_OBJECT_IRI_HTTP: &str = "http://schema.org/ImageObject";
/// <https://schema.org/ImageObject>
pub const IMAGE_OBJECT_IRI_HTTPS: &str = "https://schema.org/ImageObject";
/// <https://schema.org/ImageObject>
pub const IMAGE_OBJECT_LABEL: &str = "ImageObject";
pub struct ImageObjectIri;
impl PartialEq<&str> for ImageObjectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IMAGE_OBJECT_IRI_HTTP || *other == IMAGE_OBJECT_IRI_HTTPS
	}
}
impl PartialEq<ImageObjectIri> for &str {
	fn eq(&self, other: &ImageObjectIri) -> bool {
		*self == IMAGE_OBJECT_IRI_HTTP || *self == IMAGE_OBJECT_IRI_HTTPS
	}
}
pub struct ImageObjectIriOrLabel;
impl PartialEq<&str> for ImageObjectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ImageObjectIri || *other == IMAGE_OBJECT_LABEL
	}
}
impl PartialEq<ImageObjectIriOrLabel> for &str {
	fn eq(&self, other: &ImageObjectIriOrLabel) -> bool {
		*self == ImageObjectIri || *self == IMAGE_OBJECT_LABEL
	}
}
