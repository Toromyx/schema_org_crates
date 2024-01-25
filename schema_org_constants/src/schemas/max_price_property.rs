/// <https://schema.org/maxPrice>
pub const MAX_PRICE_PROPERTY_IRI_HTTP: &str = "http://schema.org/maxPrice";
/// <https://schema.org/maxPrice>
pub const MAX_PRICE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/maxPrice";
/// <https://schema.org/maxPrice>
pub const MAX_PRICE_PROPERTY_LABEL: &str = "maxPrice";
pub struct MaxPricePropertyIri;
impl PartialEq<&str> for MaxPricePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MAX_PRICE_PROPERTY_IRI_HTTP || *other == MAX_PRICE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MaxPricePropertyIri> for &str {
	fn eq(&self, other: &MaxPricePropertyIri) -> bool {
		*self == MAX_PRICE_PROPERTY_IRI_HTTP || *self == MAX_PRICE_PROPERTY_IRI_HTTPS
	}
}
pub struct MaxPricePropertyIriOrLabel;
impl PartialEq<&str> for MaxPricePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MaxPricePropertyIri || *other == MAX_PRICE_PROPERTY_LABEL
	}
}
impl PartialEq<MaxPricePropertyIriOrLabel> for &str {
	fn eq(&self, other: &MaxPricePropertyIriOrLabel) -> bool {
		*self == MaxPricePropertyIri || *self == MAX_PRICE_PROPERTY_LABEL
	}
}
