/// <https://schema.org/partOfOrder>
pub const PART_OF_ORDER_PROPERTY_IRI_HTTP: &str = "http://schema.org/partOfOrder";
/// <https://schema.org/partOfOrder>
pub const PART_OF_ORDER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/partOfOrder";
/// <https://schema.org/partOfOrder>
pub const PART_OF_ORDER_PROPERTY_LABEL: &str = "partOfOrder";
pub struct PartOfOrderPropertyIri;
impl PartialEq<&str> for PartOfOrderPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PART_OF_ORDER_PROPERTY_IRI_HTTP || *other == PART_OF_ORDER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PartOfOrderPropertyIri> for &str {
	fn eq(&self, other: &PartOfOrderPropertyIri) -> bool {
		*self == PART_OF_ORDER_PROPERTY_IRI_HTTP || *self == PART_OF_ORDER_PROPERTY_IRI_HTTPS
	}
}
pub struct PartOfOrderPropertyIriOrLabel;
impl PartialEq<&str> for PartOfOrderPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PartOfOrderPropertyIri || *other == PART_OF_ORDER_PROPERTY_LABEL
	}
}
impl PartialEq<PartOfOrderPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PartOfOrderPropertyIriOrLabel) -> bool {
		*self == PartOfOrderPropertyIri || *self == PART_OF_ORDER_PROPERTY_LABEL
	}
}
