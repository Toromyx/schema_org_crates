/// <https://schema.org/priceValidUntil>
pub const PRICE_VALID_UNTIL_PROPERTY_IRI_HTTP: &str = "http://schema.org/priceValidUntil";
/// <https://schema.org/priceValidUntil>
pub const PRICE_VALID_UNTIL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/priceValidUntil";
/// <https://schema.org/priceValidUntil>
pub const PRICE_VALID_UNTIL_PROPERTY_LABEL: &str = "priceValidUntil";
pub struct PriceValidUntilPropertyIri;
impl PartialEq<&str> for PriceValidUntilPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRICE_VALID_UNTIL_PROPERTY_IRI_HTTP
			|| *other == PRICE_VALID_UNTIL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PriceValidUntilPropertyIri> for &str {
	fn eq(&self, other: &PriceValidUntilPropertyIri) -> bool {
		*self == PRICE_VALID_UNTIL_PROPERTY_IRI_HTTP
			|| *self == PRICE_VALID_UNTIL_PROPERTY_IRI_HTTPS
	}
}
pub struct PriceValidUntilPropertyIriOrLabel;
impl PartialEq<&str> for PriceValidUntilPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PriceValidUntilPropertyIri || *other == PRICE_VALID_UNTIL_PROPERTY_LABEL
	}
}
impl PartialEq<PriceValidUntilPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PriceValidUntilPropertyIriOrLabel) -> bool {
		*self == PriceValidUntilPropertyIri || *self == PRICE_VALID_UNTIL_PROPERTY_LABEL
	}
}
