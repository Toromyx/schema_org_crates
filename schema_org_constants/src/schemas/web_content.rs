/// <https://schema.org/WebContent>
pub const WEB_CONTENT_IRI_HTTP: &str = "http://schema.org/WebContent";
/// <https://schema.org/WebContent>
pub const WEB_CONTENT_IRI_HTTPS: &str = "https://schema.org/WebContent";
/// <https://schema.org/WebContent>
pub const WEB_CONTENT_LABEL: &str = "WebContent";
pub struct WebContentIri;
impl PartialEq<&str> for WebContentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEB_CONTENT_IRI_HTTP || *other == WEB_CONTENT_IRI_HTTPS
	}
}
impl PartialEq<WebContentIri> for &str {
	fn eq(&self, other: &WebContentIri) -> bool {
		*self == WEB_CONTENT_IRI_HTTP || *self == WEB_CONTENT_IRI_HTTPS
	}
}
pub struct WebContentIriOrLabel;
impl PartialEq<&str> for WebContentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WebContentIri || *other == WEB_CONTENT_LABEL
	}
}
impl PartialEq<WebContentIriOrLabel> for &str {
	fn eq(&self, other: &WebContentIriOrLabel) -> bool {
		*self == WebContentIri || *self == WEB_CONTENT_LABEL
	}
}
