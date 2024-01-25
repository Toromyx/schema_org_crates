/// <https://schema.org/URL>
pub const URL_IRI_HTTP: &str = "http://schema.org/URL";
/// <https://schema.org/URL>
pub const URL_IRI_HTTPS: &str = "https://schema.org/URL";
/// <https://schema.org/URL>
pub const URL_LABEL: &str = "URL";
pub struct UrlIri;
impl PartialEq<&str> for UrlIri {
	fn eq(&self, other: &&str) -> bool {
		*other == URL_IRI_HTTP || *other == URL_IRI_HTTPS
	}
}
impl PartialEq<UrlIri> for &str {
	fn eq(&self, other: &UrlIri) -> bool {
		*self == URL_IRI_HTTP || *self == URL_IRI_HTTPS
	}
}
pub struct UrlIriOrLabel;
impl PartialEq<&str> for UrlIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UrlIri || *other == URL_LABEL
	}
}
impl PartialEq<UrlIriOrLabel> for &str {
	fn eq(&self, other: &UrlIriOrLabel) -> bool {
		*self == UrlIri || *self == URL_LABEL
	}
}
