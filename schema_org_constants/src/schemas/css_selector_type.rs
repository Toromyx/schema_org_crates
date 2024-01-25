/// <https://schema.org/CssSelectorType>
pub const CSS_SELECTOR_TYPE_IRI_HTTP: &str = "http://schema.org/CssSelectorType";
/// <https://schema.org/CssSelectorType>
pub const CSS_SELECTOR_TYPE_IRI_HTTPS: &str = "https://schema.org/CssSelectorType";
/// <https://schema.org/CssSelectorType>
pub const CSS_SELECTOR_TYPE_LABEL: &str = "CssSelectorType";
pub struct CssSelectorTypeIri;
impl PartialEq<&str> for CssSelectorTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CSS_SELECTOR_TYPE_IRI_HTTP || *other == CSS_SELECTOR_TYPE_IRI_HTTPS
	}
}
impl PartialEq<CssSelectorTypeIri> for &str {
	fn eq(&self, other: &CssSelectorTypeIri) -> bool {
		*self == CSS_SELECTOR_TYPE_IRI_HTTP || *self == CSS_SELECTOR_TYPE_IRI_HTTPS
	}
}
pub struct CssSelectorTypeIriOrLabel;
impl PartialEq<&str> for CssSelectorTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CssSelectorTypeIri || *other == CSS_SELECTOR_TYPE_LABEL
	}
}
impl PartialEq<CssSelectorTypeIriOrLabel> for &str {
	fn eq(&self, other: &CssSelectorTypeIriOrLabel) -> bool {
		*self == CssSelectorTypeIri || *self == CSS_SELECTOR_TYPE_LABEL
	}
}
