/// <https://schema.org/CompoundPriceSpecification>
pub const COMPOUND_PRICE_SPECIFICATION_IRI_HTTP: &str =
	"http://schema.org/CompoundPriceSpecification";
/// <https://schema.org/CompoundPriceSpecification>
pub const COMPOUND_PRICE_SPECIFICATION_IRI_HTTPS: &str =
	"https://schema.org/CompoundPriceSpecification";
/// <https://schema.org/CompoundPriceSpecification>
pub const COMPOUND_PRICE_SPECIFICATION_LABEL: &str = "CompoundPriceSpecification";
pub struct CompoundPriceSpecificationIri;
impl PartialEq<&str> for CompoundPriceSpecificationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMPOUND_PRICE_SPECIFICATION_IRI_HTTP
			|| *other == COMPOUND_PRICE_SPECIFICATION_IRI_HTTPS
	}
}
impl PartialEq<CompoundPriceSpecificationIri> for &str {
	fn eq(&self, other: &CompoundPriceSpecificationIri) -> bool {
		*self == COMPOUND_PRICE_SPECIFICATION_IRI_HTTP
			|| *self == COMPOUND_PRICE_SPECIFICATION_IRI_HTTPS
	}
}
pub struct CompoundPriceSpecificationIriOrLabel;
impl PartialEq<&str> for CompoundPriceSpecificationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CompoundPriceSpecificationIri || *other == COMPOUND_PRICE_SPECIFICATION_LABEL
	}
}
impl PartialEq<CompoundPriceSpecificationIriOrLabel> for &str {
	fn eq(&self, other: &CompoundPriceSpecificationIriOrLabel) -> bool {
		*self == CompoundPriceSpecificationIri || *self == COMPOUND_PRICE_SPECIFICATION_LABEL
	}
}
