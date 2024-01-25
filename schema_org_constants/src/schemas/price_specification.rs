/// <https://schema.org/PriceSpecification>
pub const PRICE_SPECIFICATION_IRI_HTTP: &str = "http://schema.org/PriceSpecification";
/// <https://schema.org/PriceSpecification>
pub const PRICE_SPECIFICATION_IRI_HTTPS: &str = "https://schema.org/PriceSpecification";
/// <https://schema.org/PriceSpecification>
pub const PRICE_SPECIFICATION_LABEL: &str = "PriceSpecification";
pub struct PriceSpecificationIri;
impl PartialEq<&str> for PriceSpecificationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRICE_SPECIFICATION_IRI_HTTP || *other == PRICE_SPECIFICATION_IRI_HTTPS
	}
}
impl PartialEq<PriceSpecificationIri> for &str {
	fn eq(&self, other: &PriceSpecificationIri) -> bool {
		*self == PRICE_SPECIFICATION_IRI_HTTP || *self == PRICE_SPECIFICATION_IRI_HTTPS
	}
}
pub struct PriceSpecificationIriOrLabel;
impl PartialEq<&str> for PriceSpecificationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PriceSpecificationIri || *other == PRICE_SPECIFICATION_LABEL
	}
}
impl PartialEq<PriceSpecificationIriOrLabel> for &str {
	fn eq(&self, other: &PriceSpecificationIriOrLabel) -> bool {
		*self == PriceSpecificationIri || *self == PRICE_SPECIFICATION_LABEL
	}
}
