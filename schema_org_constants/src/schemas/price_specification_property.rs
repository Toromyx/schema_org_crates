/// <https://schema.org/priceSpecification>
pub const PRICE_SPECIFICATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/priceSpecification";
/// <https://schema.org/priceSpecification>
pub const PRICE_SPECIFICATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/priceSpecification";
/// <https://schema.org/priceSpecification>
pub const PRICE_SPECIFICATION_PROPERTY_LABEL: &str = "priceSpecification";
pub struct PriceSpecificationPropertyIri;
impl PartialEq<&str> for PriceSpecificationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRICE_SPECIFICATION_PROPERTY_IRI_HTTP
			|| *other == PRICE_SPECIFICATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PriceSpecificationPropertyIri> for &str {
	fn eq(&self, other: &PriceSpecificationPropertyIri) -> bool {
		*self == PRICE_SPECIFICATION_PROPERTY_IRI_HTTP
			|| *self == PRICE_SPECIFICATION_PROPERTY_IRI_HTTPS
	}
}
pub struct PriceSpecificationPropertyIriOrLabel;
impl PartialEq<&str> for PriceSpecificationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PriceSpecificationPropertyIri || *other == PRICE_SPECIFICATION_PROPERTY_LABEL
	}
}
impl PartialEq<PriceSpecificationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PriceSpecificationPropertyIriOrLabel) -> bool {
		*self == PriceSpecificationPropertyIri || *self == PRICE_SPECIFICATION_PROPERTY_LABEL
	}
}
