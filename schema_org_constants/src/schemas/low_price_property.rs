/// <https://schema.org/lowPrice>
pub const LOW_PRICE_PROPERTY_IRI_HTTP: &str = "http://schema.org/lowPrice";
/// <https://schema.org/lowPrice>
pub const LOW_PRICE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/lowPrice";
/// <https://schema.org/lowPrice>
pub const LOW_PRICE_PROPERTY_LABEL: &str = "lowPrice";
pub struct LowPricePropertyIri;
impl PartialEq<&str> for LowPricePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LOW_PRICE_PROPERTY_IRI_HTTP || *other == LOW_PRICE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LowPricePropertyIri> for &str {
	fn eq(&self, other: &LowPricePropertyIri) -> bool {
		*self == LOW_PRICE_PROPERTY_IRI_HTTP || *self == LOW_PRICE_PROPERTY_IRI_HTTPS
	}
}
pub struct LowPricePropertyIriOrLabel;
impl PartialEq<&str> for LowPricePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LowPricePropertyIri || *other == LOW_PRICE_PROPERTY_LABEL
	}
}
impl PartialEq<LowPricePropertyIriOrLabel> for &str {
	fn eq(&self, other: &LowPricePropertyIriOrLabel) -> bool {
		*self == LowPricePropertyIri || *self == LOW_PRICE_PROPERTY_LABEL
	}
}
