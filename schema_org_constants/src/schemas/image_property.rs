/// <https://schema.org/image>
pub const IMAGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/image";
/// <https://schema.org/image>
pub const IMAGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/image";
/// <https://schema.org/image>
pub const IMAGE_PROPERTY_LABEL: &str = "image";
pub struct ImagePropertyIri;
impl PartialEq<&str> for ImagePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IMAGE_PROPERTY_IRI_HTTP || *other == IMAGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ImagePropertyIri> for &str {
	fn eq(&self, other: &ImagePropertyIri) -> bool {
		*self == IMAGE_PROPERTY_IRI_HTTP || *self == IMAGE_PROPERTY_IRI_HTTPS
	}
}
pub struct ImagePropertyIriOrLabel;
impl PartialEq<&str> for ImagePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ImagePropertyIri || *other == IMAGE_PROPERTY_LABEL
	}
}
impl PartialEq<ImagePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ImagePropertyIriOrLabel) -> bool {
		*self == ImagePropertyIri || *self == IMAGE_PROPERTY_LABEL
	}
}
