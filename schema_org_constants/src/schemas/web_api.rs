/// <https://schema.org/WebAPI>
pub const WEB_API_IRI_HTTP: &str = "http://schema.org/WebAPI";
/// <https://schema.org/WebAPI>
pub const WEB_API_IRI_HTTPS: &str = "https://schema.org/WebAPI";
/// <https://schema.org/WebAPI>
pub const WEB_API_LABEL: &str = "WebAPI";
pub struct WebApiIri;
impl PartialEq<&str> for WebApiIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEB_API_IRI_HTTP || *other == WEB_API_IRI_HTTPS
	}
}
impl PartialEq<WebApiIri> for &str {
	fn eq(&self, other: &WebApiIri) -> bool {
		*self == WEB_API_IRI_HTTP || *self == WEB_API_IRI_HTTPS
	}
}
pub struct WebApiIriOrLabel;
impl PartialEq<&str> for WebApiIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WebApiIri || *other == WEB_API_LABEL
	}
}
impl PartialEq<WebApiIriOrLabel> for &str {
	fn eq(&self, other: &WebApiIriOrLabel) -> bool {
		*self == WebApiIri || *self == WEB_API_LABEL
	}
}
