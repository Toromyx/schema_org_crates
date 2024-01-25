/// <https://schema.org/isBasedOnUrl>
#[deprecated = "This schema is superseded by <https://schema.org/isBasedOn>."]
pub const IS_BASED_ON_URL_PROPERTY_IRI_HTTP: &str = "http://schema.org/isBasedOnUrl";
/// <https://schema.org/isBasedOnUrl>
#[deprecated = "This schema is superseded by <https://schema.org/isBasedOn>."]
pub const IS_BASED_ON_URL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/isBasedOnUrl";
/// <https://schema.org/isBasedOnUrl>
#[deprecated = "This schema is superseded by <https://schema.org/isBasedOn>."]
pub const IS_BASED_ON_URL_PROPERTY_LABEL: &str = "isBasedOnUrl";
pub struct IsBasedOnUrlPropertyIri;
impl PartialEq<&str> for IsBasedOnUrlPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IS_BASED_ON_URL_PROPERTY_IRI_HTTP || *other == IS_BASED_ON_URL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsBasedOnUrlPropertyIri> for &str {
	fn eq(&self, other: &IsBasedOnUrlPropertyIri) -> bool {
		*self == IS_BASED_ON_URL_PROPERTY_IRI_HTTP || *self == IS_BASED_ON_URL_PROPERTY_IRI_HTTPS
	}
}
pub struct IsBasedOnUrlPropertyIriOrLabel;
impl PartialEq<&str> for IsBasedOnUrlPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsBasedOnUrlPropertyIri || *other == IS_BASED_ON_URL_PROPERTY_LABEL
	}
}
impl PartialEq<IsBasedOnUrlPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsBasedOnUrlPropertyIriOrLabel) -> bool {
		*self == IsBasedOnUrlPropertyIri || *self == IS_BASED_ON_URL_PROPERTY_LABEL
	}
}
