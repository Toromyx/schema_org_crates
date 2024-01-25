/// <https://schema.org/totalPrice>
pub const TOTAL_PRICE_PROPERTY_IRI_HTTP: &str = "http://schema.org/totalPrice";
/// <https://schema.org/totalPrice>
pub const TOTAL_PRICE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/totalPrice";
/// <https://schema.org/totalPrice>
pub const TOTAL_PRICE_PROPERTY_LABEL: &str = "totalPrice";
pub struct TotalPricePropertyIri;
impl PartialEq<&str> for TotalPricePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TOTAL_PRICE_PROPERTY_IRI_HTTP || *other == TOTAL_PRICE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TotalPricePropertyIri> for &str {
	fn eq(&self, other: &TotalPricePropertyIri) -> bool {
		*self == TOTAL_PRICE_PROPERTY_IRI_HTTP || *self == TOTAL_PRICE_PROPERTY_IRI_HTTPS
	}
}
pub struct TotalPricePropertyIriOrLabel;
impl PartialEq<&str> for TotalPricePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TotalPricePropertyIri || *other == TOTAL_PRICE_PROPERTY_LABEL
	}
}
impl PartialEq<TotalPricePropertyIriOrLabel> for &str {
	fn eq(&self, other: &TotalPricePropertyIriOrLabel) -> bool {
		*self == TotalPricePropertyIri || *self == TOTAL_PRICE_PROPERTY_LABEL
	}
}
