/// <https://schema.org/PriceComponentTypeEnumeration>
pub const PRICE_COMPONENT_TYPE_ENUMERATION_IRI_HTTP: &str =
	"http://schema.org/PriceComponentTypeEnumeration";
/// <https://schema.org/PriceComponentTypeEnumeration>
pub const PRICE_COMPONENT_TYPE_ENUMERATION_IRI_HTTPS: &str =
	"https://schema.org/PriceComponentTypeEnumeration";
/// <https://schema.org/PriceComponentTypeEnumeration>
pub const PRICE_COMPONENT_TYPE_ENUMERATION_LABEL: &str = "PriceComponentTypeEnumeration";
pub struct PriceComponentTypeEnumerationIri;
impl PartialEq<&str> for PriceComponentTypeEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRICE_COMPONENT_TYPE_ENUMERATION_IRI_HTTP
			|| *other == PRICE_COMPONENT_TYPE_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<PriceComponentTypeEnumerationIri> for &str {
	fn eq(&self, other: &PriceComponentTypeEnumerationIri) -> bool {
		*self == PRICE_COMPONENT_TYPE_ENUMERATION_IRI_HTTP
			|| *self == PRICE_COMPONENT_TYPE_ENUMERATION_IRI_HTTPS
	}
}
pub struct PriceComponentTypeEnumerationIriOrLabel;
impl PartialEq<&str> for PriceComponentTypeEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PriceComponentTypeEnumerationIri
			|| *other == PRICE_COMPONENT_TYPE_ENUMERATION_LABEL
	}
}
impl PartialEq<PriceComponentTypeEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &PriceComponentTypeEnumerationIriOrLabel) -> bool {
		*self == PriceComponentTypeEnumerationIri || *self == PRICE_COMPONENT_TYPE_ENUMERATION_LABEL
	}
}
