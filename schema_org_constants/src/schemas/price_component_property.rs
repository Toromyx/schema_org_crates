/// <https://schema.org/priceComponent>
pub const PRICE_COMPONENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/priceComponent";
/// <https://schema.org/priceComponent>
pub const PRICE_COMPONENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/priceComponent";
/// <https://schema.org/priceComponent>
pub const PRICE_COMPONENT_PROPERTY_LABEL: &str = "priceComponent";
pub struct PriceComponentPropertyIri;
impl PartialEq<&str> for PriceComponentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRICE_COMPONENT_PROPERTY_IRI_HTTP || *other == PRICE_COMPONENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PriceComponentPropertyIri> for &str {
	fn eq(&self, other: &PriceComponentPropertyIri) -> bool {
		*self == PRICE_COMPONENT_PROPERTY_IRI_HTTP || *self == PRICE_COMPONENT_PROPERTY_IRI_HTTPS
	}
}
pub struct PriceComponentPropertyIriOrLabel;
impl PartialEq<&str> for PriceComponentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PriceComponentPropertyIri || *other == PRICE_COMPONENT_PROPERTY_LABEL
	}
}
impl PartialEq<PriceComponentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PriceComponentPropertyIriOrLabel) -> bool {
		*self == PriceComponentPropertyIri || *self == PRICE_COMPONENT_PROPERTY_LABEL
	}
}
