/// <https://schema.org/urlTemplate>
pub const URL_TEMPLATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/urlTemplate";
/// <https://schema.org/urlTemplate>
pub const URL_TEMPLATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/urlTemplate";
/// <https://schema.org/urlTemplate>
pub const URL_TEMPLATE_PROPERTY_LABEL: &str = "urlTemplate";
pub struct UrlTemplatePropertyIri;
impl PartialEq<&str> for UrlTemplatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == URL_TEMPLATE_PROPERTY_IRI_HTTP || *other == URL_TEMPLATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<UrlTemplatePropertyIri> for &str {
	fn eq(&self, other: &UrlTemplatePropertyIri) -> bool {
		*self == URL_TEMPLATE_PROPERTY_IRI_HTTP || *self == URL_TEMPLATE_PROPERTY_IRI_HTTPS
	}
}
pub struct UrlTemplatePropertyIriOrLabel;
impl PartialEq<&str> for UrlTemplatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UrlTemplatePropertyIri || *other == URL_TEMPLATE_PROPERTY_LABEL
	}
}
impl PartialEq<UrlTemplatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &UrlTemplatePropertyIriOrLabel) -> bool {
		*self == UrlTemplatePropertyIri || *self == URL_TEMPLATE_PROPERTY_LABEL
	}
}
