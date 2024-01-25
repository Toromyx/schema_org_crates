/// <https://schema.org/priceRange>
pub const PRICE_RANGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/priceRange";
/// <https://schema.org/priceRange>
pub const PRICE_RANGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/priceRange";
/// <https://schema.org/priceRange>
pub const PRICE_RANGE_PROPERTY_LABEL: &str = "priceRange";
pub struct PriceRangePropertyIri;
impl PartialEq<&str> for PriceRangePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRICE_RANGE_PROPERTY_IRI_HTTP || *other == PRICE_RANGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PriceRangePropertyIri> for &str {
	fn eq(&self, other: &PriceRangePropertyIri) -> bool {
		*self == PRICE_RANGE_PROPERTY_IRI_HTTP || *self == PRICE_RANGE_PROPERTY_IRI_HTTPS
	}
}
pub struct PriceRangePropertyIriOrLabel;
impl PartialEq<&str> for PriceRangePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PriceRangePropertyIri || *other == PRICE_RANGE_PROPERTY_LABEL
	}
}
impl PartialEq<PriceRangePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PriceRangePropertyIriOrLabel) -> bool {
		*self == PriceRangePropertyIri || *self == PRICE_RANGE_PROPERTY_LABEL
	}
}
