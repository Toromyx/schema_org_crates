/// <https://schema.org/WebSite>
pub const WEB_SITE_IRI_HTTP: &str = "http://schema.org/WebSite";
/// <https://schema.org/WebSite>
pub const WEB_SITE_IRI_HTTPS: &str = "https://schema.org/WebSite";
/// <https://schema.org/WebSite>
pub const WEB_SITE_LABEL: &str = "WebSite";
pub struct WebSiteIri;
impl PartialEq<&str> for WebSiteIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEB_SITE_IRI_HTTP || *other == WEB_SITE_IRI_HTTPS
	}
}
impl PartialEq<WebSiteIri> for &str {
	fn eq(&self, other: &WebSiteIri) -> bool {
		*self == WEB_SITE_IRI_HTTP || *self == WEB_SITE_IRI_HTTPS
	}
}
pub struct WebSiteIriOrLabel;
impl PartialEq<&str> for WebSiteIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WebSiteIri || *other == WEB_SITE_LABEL
	}
}
impl PartialEq<WebSiteIriOrLabel> for &str {
	fn eq(&self, other: &WebSiteIriOrLabel) -> bool {
		*self == WebSiteIri || *self == WEB_SITE_LABEL
	}
}
