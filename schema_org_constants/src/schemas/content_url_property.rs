/// <https://schema.org/contentUrl>
pub const CONTENT_URL_PROPERTY_IRI_HTTP: &str = "http://schema.org/contentUrl";
/// <https://schema.org/contentUrl>
pub const CONTENT_URL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/contentUrl";
/// <https://schema.org/contentUrl>
pub const CONTENT_URL_PROPERTY_LABEL: &str = "contentUrl";
pub struct ContentUrlPropertyIri;
impl PartialEq<&str> for ContentUrlPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTENT_URL_PROPERTY_IRI_HTTP || *other == CONTENT_URL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ContentUrlPropertyIri> for &str {
	fn eq(&self, other: &ContentUrlPropertyIri) -> bool {
		*self == CONTENT_URL_PROPERTY_IRI_HTTP || *self == CONTENT_URL_PROPERTY_IRI_HTTPS
	}
}
pub struct ContentUrlPropertyIriOrLabel;
impl PartialEq<&str> for ContentUrlPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ContentUrlPropertyIri || *other == CONTENT_URL_PROPERTY_LABEL
	}
}
impl PartialEq<ContentUrlPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ContentUrlPropertyIriOrLabel) -> bool {
		*self == ContentUrlPropertyIri || *self == CONTENT_URL_PROPERTY_LABEL
	}
}
