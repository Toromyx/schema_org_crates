/// <https://schema.org/layoutImage>
pub const LAYOUT_IMAGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/layoutImage";
/// <https://schema.org/layoutImage>
pub const LAYOUT_IMAGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/layoutImage";
/// <https://schema.org/layoutImage>
pub const LAYOUT_IMAGE_PROPERTY_LABEL: &str = "layoutImage";
pub struct LayoutImagePropertyIri;
impl PartialEq<&str> for LayoutImagePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LAYOUT_IMAGE_PROPERTY_IRI_HTTP || *other == LAYOUT_IMAGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LayoutImagePropertyIri> for &str {
	fn eq(&self, other: &LayoutImagePropertyIri) -> bool {
		*self == LAYOUT_IMAGE_PROPERTY_IRI_HTTP || *self == LAYOUT_IMAGE_PROPERTY_IRI_HTTPS
	}
}
pub struct LayoutImagePropertyIriOrLabel;
impl PartialEq<&str> for LayoutImagePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LayoutImagePropertyIri || *other == LAYOUT_IMAGE_PROPERTY_LABEL
	}
}
impl PartialEq<LayoutImagePropertyIriOrLabel> for &str {
	fn eq(&self, other: &LayoutImagePropertyIriOrLabel) -> bool {
		*self == LayoutImagePropertyIri || *self == LAYOUT_IMAGE_PROPERTY_LABEL
	}
}
