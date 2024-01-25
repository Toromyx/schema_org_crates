/// <https://schema.org/cssSelector>
pub const CSS_SELECTOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/cssSelector";
/// <https://schema.org/cssSelector>
pub const CSS_SELECTOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/cssSelector";
/// <https://schema.org/cssSelector>
pub const CSS_SELECTOR_PROPERTY_LABEL: &str = "cssSelector";
pub struct CssSelectorPropertyIri;
impl PartialEq<&str> for CssSelectorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CSS_SELECTOR_PROPERTY_IRI_HTTP || *other == CSS_SELECTOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CssSelectorPropertyIri> for &str {
	fn eq(&self, other: &CssSelectorPropertyIri) -> bool {
		*self == CSS_SELECTOR_PROPERTY_IRI_HTTP || *self == CSS_SELECTOR_PROPERTY_IRI_HTTPS
	}
}
pub struct CssSelectorPropertyIriOrLabel;
impl PartialEq<&str> for CssSelectorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CssSelectorPropertyIri || *other == CSS_SELECTOR_PROPERTY_LABEL
	}
}
impl PartialEq<CssSelectorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CssSelectorPropertyIriOrLabel) -> bool {
		*self == CssSelectorPropertyIri || *self == CSS_SELECTOR_PROPERTY_LABEL
	}
}
