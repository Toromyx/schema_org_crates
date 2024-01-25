/// <https://schema.org/WebPage>
pub const WEB_PAGE_IRI_HTTP: &str = "http://schema.org/WebPage";
/// <https://schema.org/WebPage>
pub const WEB_PAGE_IRI_HTTPS: &str = "https://schema.org/WebPage";
/// <https://schema.org/WebPage>
pub const WEB_PAGE_LABEL: &str = "WebPage";
pub struct WebPageIri;
impl PartialEq<&str> for WebPageIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEB_PAGE_IRI_HTTP || *other == WEB_PAGE_IRI_HTTPS
	}
}
impl PartialEq<WebPageIri> for &str {
	fn eq(&self, other: &WebPageIri) -> bool {
		*self == WEB_PAGE_IRI_HTTP || *self == WEB_PAGE_IRI_HTTPS
	}
}
pub struct WebPageIriOrLabel;
impl PartialEq<&str> for WebPageIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WebPageIri || *other == WEB_PAGE_LABEL
	}
}
impl PartialEq<WebPageIriOrLabel> for &str {
	fn eq(&self, other: &WebPageIriOrLabel) -> bool {
		*self == WebPageIri || *self == WEB_PAGE_LABEL
	}
}
