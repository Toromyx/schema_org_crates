/// <https://schema.org/currency>
pub const CURRENCY_PROPERTY_IRI_HTTP: &str = "http://schema.org/currency";
/// <https://schema.org/currency>
pub const CURRENCY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/currency";
/// <https://schema.org/currency>
pub const CURRENCY_PROPERTY_LABEL: &str = "currency";
pub struct CurrencyPropertyIri;
impl PartialEq<&str> for CurrencyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CURRENCY_PROPERTY_IRI_HTTP || *other == CURRENCY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CurrencyPropertyIri> for &str {
	fn eq(&self, other: &CurrencyPropertyIri) -> bool {
		*self == CURRENCY_PROPERTY_IRI_HTTP || *self == CURRENCY_PROPERTY_IRI_HTTPS
	}
}
pub struct CurrencyPropertyIriOrLabel;
impl PartialEq<&str> for CurrencyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CurrencyPropertyIri || *other == CURRENCY_PROPERTY_LABEL
	}
}
impl PartialEq<CurrencyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CurrencyPropertyIriOrLabel) -> bool {
		*self == CurrencyPropertyIri || *self == CURRENCY_PROPERTY_LABEL
	}
}
