/// <https://schema.org/contentType>
pub const CONTENT_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/contentType";
/// <https://schema.org/contentType>
pub const CONTENT_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/contentType";
/// <https://schema.org/contentType>
pub const CONTENT_TYPE_PROPERTY_LABEL: &str = "contentType";
pub struct ContentTypePropertyIri;
impl PartialEq<&str> for ContentTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTENT_TYPE_PROPERTY_IRI_HTTP || *other == CONTENT_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ContentTypePropertyIri> for &str {
	fn eq(&self, other: &ContentTypePropertyIri) -> bool {
		*self == CONTENT_TYPE_PROPERTY_IRI_HTTP || *self == CONTENT_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct ContentTypePropertyIriOrLabel;
impl PartialEq<&str> for ContentTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ContentTypePropertyIri || *other == CONTENT_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<ContentTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ContentTypePropertyIriOrLabel) -> bool {
		*self == ContentTypePropertyIri || *self == CONTENT_TYPE_PROPERTY_LABEL
	}
}
