/// <https://schema.org/option>
#[deprecated = "This schema is superseded by <https://schema.org/actionOption>."]
pub const OPTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/option";
/// <https://schema.org/option>
#[deprecated = "This schema is superseded by <https://schema.org/actionOption>."]
pub const OPTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/option";
/// <https://schema.org/option>
#[deprecated = "This schema is superseded by <https://schema.org/actionOption>."]
pub const OPTION_PROPERTY_LABEL: &str = "option";
pub struct OptionPropertyIri;
impl PartialEq<&str> for OptionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OPTION_PROPERTY_IRI_HTTP || *other == OPTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OptionPropertyIri> for &str {
	fn eq(&self, other: &OptionPropertyIri) -> bool {
		*self == OPTION_PROPERTY_IRI_HTTP || *self == OPTION_PROPERTY_IRI_HTTPS
	}
}
pub struct OptionPropertyIriOrLabel;
impl PartialEq<&str> for OptionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OptionPropertyIri || *other == OPTION_PROPERTY_LABEL
	}
}
impl PartialEq<OptionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OptionPropertyIriOrLabel) -> bool {
		*self == OptionPropertyIri || *self == OPTION_PROPERTY_LABEL
	}
}
