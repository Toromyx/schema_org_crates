/// <https://schema.org/url>
pub const URL_PROPERTY_IRI_HTTP: &str = "http://schema.org/url";
/// <https://schema.org/url>
pub const URL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/url";
/// <https://schema.org/url>
pub const URL_PROPERTY_LABEL: &str = "url";
pub struct UrlPropertyIri;
impl PartialEq<&str> for UrlPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == URL_PROPERTY_IRI_HTTP || *other == URL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<UrlPropertyIri> for &str {
	fn eq(&self, other: &UrlPropertyIri) -> bool {
		*self == URL_PROPERTY_IRI_HTTP || *self == URL_PROPERTY_IRI_HTTPS
	}
}
pub struct UrlPropertyIriOrLabel;
impl PartialEq<&str> for UrlPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UrlPropertyIri || *other == URL_PROPERTY_LABEL
	}
}
impl PartialEq<UrlPropertyIriOrLabel> for &str {
	fn eq(&self, other: &UrlPropertyIriOrLabel) -> bool {
		*self == UrlPropertyIri || *self == URL_PROPERTY_LABEL
	}
}
