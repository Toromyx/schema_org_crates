/// <https://schema.org/discountCurrency>
pub const DISCOUNT_CURRENCY_PROPERTY_IRI_HTTP: &str = "http://schema.org/discountCurrency";
/// <https://schema.org/discountCurrency>
pub const DISCOUNT_CURRENCY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/discountCurrency";
/// <https://schema.org/discountCurrency>
pub const DISCOUNT_CURRENCY_PROPERTY_LABEL: &str = "discountCurrency";
pub struct DiscountCurrencyPropertyIri;
impl PartialEq<&str> for DiscountCurrencyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DISCOUNT_CURRENCY_PROPERTY_IRI_HTTP
			|| *other == DISCOUNT_CURRENCY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DiscountCurrencyPropertyIri> for &str {
	fn eq(&self, other: &DiscountCurrencyPropertyIri) -> bool {
		*self == DISCOUNT_CURRENCY_PROPERTY_IRI_HTTP
			|| *self == DISCOUNT_CURRENCY_PROPERTY_IRI_HTTPS
	}
}
pub struct DiscountCurrencyPropertyIriOrLabel;
impl PartialEq<&str> for DiscountCurrencyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DiscountCurrencyPropertyIri || *other == DISCOUNT_CURRENCY_PROPERTY_LABEL
	}
}
impl PartialEq<DiscountCurrencyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DiscountCurrencyPropertyIriOrLabel) -> bool {
		*self == DiscountCurrencyPropertyIri || *self == DISCOUNT_CURRENCY_PROPERTY_LABEL
	}
}
