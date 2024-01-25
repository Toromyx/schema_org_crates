/// <https://schema.org/priceType>
pub const PRICE_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/priceType";
/// <https://schema.org/priceType>
pub const PRICE_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/priceType";
/// <https://schema.org/priceType>
pub const PRICE_TYPE_PROPERTY_LABEL: &str = "priceType";
pub struct PriceTypePropertyIri;
impl PartialEq<&str> for PriceTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRICE_TYPE_PROPERTY_IRI_HTTP || *other == PRICE_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PriceTypePropertyIri> for &str {
	fn eq(&self, other: &PriceTypePropertyIri) -> bool {
		*self == PRICE_TYPE_PROPERTY_IRI_HTTP || *self == PRICE_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct PriceTypePropertyIriOrLabel;
impl PartialEq<&str> for PriceTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PriceTypePropertyIri || *other == PRICE_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<PriceTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PriceTypePropertyIriOrLabel) -> bool {
		*self == PriceTypePropertyIri || *self == PRICE_TYPE_PROPERTY_LABEL
	}
}
