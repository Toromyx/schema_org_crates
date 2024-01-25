/// <https://schema.org/PriceTypeEnumeration>
pub const PRICE_TYPE_ENUMERATION_IRI_HTTP: &str = "http://schema.org/PriceTypeEnumeration";
/// <https://schema.org/PriceTypeEnumeration>
pub const PRICE_TYPE_ENUMERATION_IRI_HTTPS: &str = "https://schema.org/PriceTypeEnumeration";
/// <https://schema.org/PriceTypeEnumeration>
pub const PRICE_TYPE_ENUMERATION_LABEL: &str = "PriceTypeEnumeration";
pub struct PriceTypeEnumerationIri;
impl PartialEq<&str> for PriceTypeEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRICE_TYPE_ENUMERATION_IRI_HTTP || *other == PRICE_TYPE_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<PriceTypeEnumerationIri> for &str {
	fn eq(&self, other: &PriceTypeEnumerationIri) -> bool {
		*self == PRICE_TYPE_ENUMERATION_IRI_HTTP || *self == PRICE_TYPE_ENUMERATION_IRI_HTTPS
	}
}
pub struct PriceTypeEnumerationIriOrLabel;
impl PartialEq<&str> for PriceTypeEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PriceTypeEnumerationIri || *other == PRICE_TYPE_ENUMERATION_LABEL
	}
}
impl PartialEq<PriceTypeEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &PriceTypeEnumerationIriOrLabel) -> bool {
		*self == PriceTypeEnumerationIri || *self == PRICE_TYPE_ENUMERATION_LABEL
	}
}
