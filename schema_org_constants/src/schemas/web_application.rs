/// <https://schema.org/WebApplication>
pub const WEB_APPLICATION_IRI_HTTP: &str = "http://schema.org/WebApplication";
/// <https://schema.org/WebApplication>
pub const WEB_APPLICATION_IRI_HTTPS: &str = "https://schema.org/WebApplication";
/// <https://schema.org/WebApplication>
pub const WEB_APPLICATION_LABEL: &str = "WebApplication";
pub struct WebApplicationIri;
impl PartialEq<&str> for WebApplicationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEB_APPLICATION_IRI_HTTP || *other == WEB_APPLICATION_IRI_HTTPS
	}
}
impl PartialEq<WebApplicationIri> for &str {
	fn eq(&self, other: &WebApplicationIri) -> bool {
		*self == WEB_APPLICATION_IRI_HTTP || *self == WEB_APPLICATION_IRI_HTTPS
	}
}
pub struct WebApplicationIriOrLabel;
impl PartialEq<&str> for WebApplicationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WebApplicationIri || *other == WEB_APPLICATION_LABEL
	}
}
impl PartialEq<WebApplicationIriOrLabel> for &str {
	fn eq(&self, other: &WebApplicationIriOrLabel) -> bool {
		*self == WebApplicationIri || *self == WEB_APPLICATION_LABEL
	}
}
