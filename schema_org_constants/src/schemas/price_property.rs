/// <https://schema.org/price>
pub const PRICE_PROPERTY_IRI_HTTP: &str = "http://schema.org/price";
/// <https://schema.org/price>
pub const PRICE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/price";
/// <https://schema.org/price>
pub const PRICE_PROPERTY_LABEL: &str = "price";
pub struct PricePropertyIri;
impl PartialEq<&str> for PricePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRICE_PROPERTY_IRI_HTTP || *other == PRICE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PricePropertyIri> for &str {
	fn eq(&self, other: &PricePropertyIri) -> bool {
		*self == PRICE_PROPERTY_IRI_HTTP || *self == PRICE_PROPERTY_IRI_HTTPS
	}
}
pub struct PricePropertyIriOrLabel;
impl PartialEq<&str> for PricePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PricePropertyIri || *other == PRICE_PROPERTY_LABEL
	}
}
impl PartialEq<PricePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PricePropertyIriOrLabel) -> bool {
		*self == PricePropertyIri || *self == PRICE_PROPERTY_LABEL
	}
}
