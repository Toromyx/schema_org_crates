/// <https://schema.org/WebPageElement>
pub const WEB_PAGE_ELEMENT_IRI_HTTP: &str = "http://schema.org/WebPageElement";
/// <https://schema.org/WebPageElement>
pub const WEB_PAGE_ELEMENT_IRI_HTTPS: &str = "https://schema.org/WebPageElement";
/// <https://schema.org/WebPageElement>
pub const WEB_PAGE_ELEMENT_LABEL: &str = "WebPageElement";
pub struct WebPageElementIri;
impl PartialEq<&str> for WebPageElementIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEB_PAGE_ELEMENT_IRI_HTTP || *other == WEB_PAGE_ELEMENT_IRI_HTTPS
	}
}
impl PartialEq<WebPageElementIri> for &str {
	fn eq(&self, other: &WebPageElementIri) -> bool {
		*self == WEB_PAGE_ELEMENT_IRI_HTTP || *self == WEB_PAGE_ELEMENT_IRI_HTTPS
	}
}
pub struct WebPageElementIriOrLabel;
impl PartialEq<&str> for WebPageElementIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WebPageElementIri || *other == WEB_PAGE_ELEMENT_LABEL
	}
}
impl PartialEq<WebPageElementIriOrLabel> for &str {
	fn eq(&self, other: &WebPageElementIriOrLabel) -> bool {
		*self == WebPageElementIri || *self == WEB_PAGE_ELEMENT_LABEL
	}
}
