/// <https://schema.org/contentSize>
pub const CONTENT_SIZE_PROPERTY_IRI_HTTP: &str = "http://schema.org/contentSize";
/// <https://schema.org/contentSize>
pub const CONTENT_SIZE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/contentSize";
/// <https://schema.org/contentSize>
pub const CONTENT_SIZE_PROPERTY_LABEL: &str = "contentSize";
pub struct ContentSizePropertyIri;
impl PartialEq<&str> for ContentSizePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTENT_SIZE_PROPERTY_IRI_HTTP || *other == CONTENT_SIZE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ContentSizePropertyIri> for &str {
	fn eq(&self, other: &ContentSizePropertyIri) -> bool {
		*self == CONTENT_SIZE_PROPERTY_IRI_HTTP || *self == CONTENT_SIZE_PROPERTY_IRI_HTTPS
	}
}
pub struct ContentSizePropertyIriOrLabel;
impl PartialEq<&str> for ContentSizePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ContentSizePropertyIri || *other == CONTENT_SIZE_PROPERTY_LABEL
	}
}
impl PartialEq<ContentSizePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ContentSizePropertyIriOrLabel) -> bool {
		*self == ContentSizePropertyIri || *self == CONTENT_SIZE_PROPERTY_LABEL
	}
}
