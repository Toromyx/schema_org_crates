/// <https://schema.org/priceCurrency>
pub const PRICE_CURRENCY_PROPERTY_IRI_HTTP: &str = "http://schema.org/priceCurrency";
/// <https://schema.org/priceCurrency>
pub const PRICE_CURRENCY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/priceCurrency";
/// <https://schema.org/priceCurrency>
pub const PRICE_CURRENCY_PROPERTY_LABEL: &str = "priceCurrency";
pub struct PriceCurrencyPropertyIri;
impl PartialEq<&str> for PriceCurrencyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRICE_CURRENCY_PROPERTY_IRI_HTTP || *other == PRICE_CURRENCY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PriceCurrencyPropertyIri> for &str {
	fn eq(&self, other: &PriceCurrencyPropertyIri) -> bool {
		*self == PRICE_CURRENCY_PROPERTY_IRI_HTTP || *self == PRICE_CURRENCY_PROPERTY_IRI_HTTPS
	}
}
pub struct PriceCurrencyPropertyIriOrLabel;
impl PartialEq<&str> for PriceCurrencyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PriceCurrencyPropertyIri || *other == PRICE_CURRENCY_PROPERTY_LABEL
	}
}
impl PartialEq<PriceCurrencyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PriceCurrencyPropertyIriOrLabel) -> bool {
		*self == PriceCurrencyPropertyIri || *self == PRICE_CURRENCY_PROPERTY_LABEL
	}
}
