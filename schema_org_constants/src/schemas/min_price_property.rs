/// <https://schema.org/minPrice>
pub const MIN_PRICE_PROPERTY_IRI_HTTP: &str = "http://schema.org/minPrice";
/// <https://schema.org/minPrice>
pub const MIN_PRICE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/minPrice";
/// <https://schema.org/minPrice>
pub const MIN_PRICE_PROPERTY_LABEL: &str = "minPrice";
pub struct MinPricePropertyIri;
impl PartialEq<&str> for MinPricePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MIN_PRICE_PROPERTY_IRI_HTTP || *other == MIN_PRICE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MinPricePropertyIri> for &str {
	fn eq(&self, other: &MinPricePropertyIri) -> bool {
		*self == MIN_PRICE_PROPERTY_IRI_HTTP || *self == MIN_PRICE_PROPERTY_IRI_HTTPS
	}
}
pub struct MinPricePropertyIriOrLabel;
impl PartialEq<&str> for MinPricePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MinPricePropertyIri || *other == MIN_PRICE_PROPERTY_LABEL
	}
}
impl PartialEq<MinPricePropertyIriOrLabel> for &str {
	fn eq(&self, other: &MinPricePropertyIriOrLabel) -> bool {
		*self == MinPricePropertyIri || *self == MIN_PRICE_PROPERTY_LABEL
	}
}
