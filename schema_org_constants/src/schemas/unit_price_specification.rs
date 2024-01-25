/// <https://schema.org/UnitPriceSpecification>
pub const UNIT_PRICE_SPECIFICATION_IRI_HTTP: &str = "http://schema.org/UnitPriceSpecification";
/// <https://schema.org/UnitPriceSpecification>
pub const UNIT_PRICE_SPECIFICATION_IRI_HTTPS: &str = "https://schema.org/UnitPriceSpecification";
/// <https://schema.org/UnitPriceSpecification>
pub const UNIT_PRICE_SPECIFICATION_LABEL: &str = "UnitPriceSpecification";
pub struct UnitPriceSpecificationIri;
impl PartialEq<&str> for UnitPriceSpecificationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == UNIT_PRICE_SPECIFICATION_IRI_HTTP || *other == UNIT_PRICE_SPECIFICATION_IRI_HTTPS
	}
}
impl PartialEq<UnitPriceSpecificationIri> for &str {
	fn eq(&self, other: &UnitPriceSpecificationIri) -> bool {
		*self == UNIT_PRICE_SPECIFICATION_IRI_HTTP || *self == UNIT_PRICE_SPECIFICATION_IRI_HTTPS
	}
}
pub struct UnitPriceSpecificationIriOrLabel;
impl PartialEq<&str> for UnitPriceSpecificationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UnitPriceSpecificationIri || *other == UNIT_PRICE_SPECIFICATION_LABEL
	}
}
impl PartialEq<UnitPriceSpecificationIriOrLabel> for &str {
	fn eq(&self, other: &UnitPriceSpecificationIriOrLabel) -> bool {
		*self == UnitPriceSpecificationIri || *self == UNIT_PRICE_SPECIFICATION_LABEL
	}
}
