/// <https://schema.org/priceComponentType>
pub const PRICE_COMPONENT_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/priceComponentType";
/// <https://schema.org/priceComponentType>
pub const PRICE_COMPONENT_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/priceComponentType";
/// <https://schema.org/priceComponentType>
pub const PRICE_COMPONENT_TYPE_PROPERTY_LABEL: &str = "priceComponentType";
pub struct PriceComponentTypePropertyIri;
impl PartialEq<&str> for PriceComponentTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRICE_COMPONENT_TYPE_PROPERTY_IRI_HTTP
			|| *other == PRICE_COMPONENT_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PriceComponentTypePropertyIri> for &str {
	fn eq(&self, other: &PriceComponentTypePropertyIri) -> bool {
		*self == PRICE_COMPONENT_TYPE_PROPERTY_IRI_HTTP
			|| *self == PRICE_COMPONENT_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct PriceComponentTypePropertyIriOrLabel;
impl PartialEq<&str> for PriceComponentTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PriceComponentTypePropertyIri || *other == PRICE_COMPONENT_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<PriceComponentTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PriceComponentTypePropertyIriOrLabel) -> bool {
		*self == PriceComponentTypePropertyIri || *self == PRICE_COMPONENT_TYPE_PROPERTY_LABEL
	}
}
