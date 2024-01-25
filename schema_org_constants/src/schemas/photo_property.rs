/// <https://schema.org/photo>
pub const PHOTO_PROPERTY_IRI_HTTP: &str = "http://schema.org/photo";
/// <https://schema.org/photo>
pub const PHOTO_PROPERTY_IRI_HTTPS: &str = "https://schema.org/photo";
/// <https://schema.org/photo>
pub const PHOTO_PROPERTY_LABEL: &str = "photo";
pub struct PhotoPropertyIri;
impl PartialEq<&str> for PhotoPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PHOTO_PROPERTY_IRI_HTTP || *other == PHOTO_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PhotoPropertyIri> for &str {
	fn eq(&self, other: &PhotoPropertyIri) -> bool {
		*self == PHOTO_PROPERTY_IRI_HTTP || *self == PHOTO_PROPERTY_IRI_HTTPS
	}
}
pub struct PhotoPropertyIriOrLabel;
impl PartialEq<&str> for PhotoPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PhotoPropertyIri || *other == PHOTO_PROPERTY_LABEL
	}
}
impl PartialEq<PhotoPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PhotoPropertyIriOrLabel) -> bool {
		*self == PhotoPropertyIri || *self == PHOTO_PROPERTY_LABEL
	}
}
