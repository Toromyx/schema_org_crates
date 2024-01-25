/// <https://schema.org/photos>
#[deprecated = "This schema is superseded by <https://schema.org/photo>."]
pub const PHOTOS_PROPERTY_IRI_HTTP: &str = "http://schema.org/photos";
/// <https://schema.org/photos>
#[deprecated = "This schema is superseded by <https://schema.org/photo>."]
pub const PHOTOS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/photos";
/// <https://schema.org/photos>
#[deprecated = "This schema is superseded by <https://schema.org/photo>."]
pub const PHOTOS_PROPERTY_LABEL: &str = "photos";
pub struct PhotosPropertyIri;
impl PartialEq<&str> for PhotosPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PHOTOS_PROPERTY_IRI_HTTP || *other == PHOTOS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PhotosPropertyIri> for &str {
	fn eq(&self, other: &PhotosPropertyIri) -> bool {
		*self == PHOTOS_PROPERTY_IRI_HTTP || *self == PHOTOS_PROPERTY_IRI_HTTPS
	}
}
pub struct PhotosPropertyIriOrLabel;
impl PartialEq<&str> for PhotosPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PhotosPropertyIri || *other == PHOTOS_PROPERTY_LABEL
	}
}
impl PartialEq<PhotosPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PhotosPropertyIriOrLabel) -> bool {
		*self == PhotosPropertyIri || *self == PHOTOS_PROPERTY_LABEL
	}
}
