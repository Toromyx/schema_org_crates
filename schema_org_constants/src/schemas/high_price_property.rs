/// <https://schema.org/highPrice>
pub const HIGH_PRICE_PROPERTY_IRI_HTTP: &str = "http://schema.org/highPrice";
/// <https://schema.org/highPrice>
pub const HIGH_PRICE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/highPrice";
/// <https://schema.org/highPrice>
pub const HIGH_PRICE_PROPERTY_LABEL: &str = "highPrice";
pub struct HighPricePropertyIri;
impl PartialEq<&str> for HighPricePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HIGH_PRICE_PROPERTY_IRI_HTTP || *other == HIGH_PRICE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HighPricePropertyIri> for &str {
	fn eq(&self, other: &HighPricePropertyIri) -> bool {
		*self == HIGH_PRICE_PROPERTY_IRI_HTTP || *self == HIGH_PRICE_PROPERTY_IRI_HTTPS
	}
}
pub struct HighPricePropertyIriOrLabel;
impl PartialEq<&str> for HighPricePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HighPricePropertyIri || *other == HIGH_PRICE_PROPERTY_LABEL
	}
}
impl PartialEq<HighPricePropertyIriOrLabel> for &str {
	fn eq(&self, other: &HighPricePropertyIriOrLabel) -> bool {
		*self == HighPricePropertyIri || *self == HIGH_PRICE_PROPERTY_LABEL
	}
}
