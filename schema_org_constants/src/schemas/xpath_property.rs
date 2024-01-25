/// <https://schema.org/xpath>
pub const XPATH_PROPERTY_IRI_HTTP: &str = "http://schema.org/xpath";
/// <https://schema.org/xpath>
pub const XPATH_PROPERTY_IRI_HTTPS: &str = "https://schema.org/xpath";
/// <https://schema.org/xpath>
pub const XPATH_PROPERTY_LABEL: &str = "xpath";
pub struct XpathPropertyIri;
impl PartialEq<&str> for XpathPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == XPATH_PROPERTY_IRI_HTTP || *other == XPATH_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<XpathPropertyIri> for &str {
	fn eq(&self, other: &XpathPropertyIri) -> bool {
		*self == XPATH_PROPERTY_IRI_HTTP || *self == XPATH_PROPERTY_IRI_HTTPS
	}
}
pub struct XpathPropertyIriOrLabel;
impl PartialEq<&str> for XpathPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == XpathPropertyIri || *other == XPATH_PROPERTY_LABEL
	}
}
impl PartialEq<XpathPropertyIriOrLabel> for &str {
	fn eq(&self, other: &XpathPropertyIriOrLabel) -> bool {
		*self == XpathPropertyIri || *self == XPATH_PROPERTY_LABEL
	}
}
