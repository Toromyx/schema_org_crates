/// <https://schema.org/MerchantReturnEnumeration>
pub const MERCHANT_RETURN_ENUMERATION_IRI_HTTP: &str =
	"http://schema.org/MerchantReturnEnumeration";
/// <https://schema.org/MerchantReturnEnumeration>
pub const MERCHANT_RETURN_ENUMERATION_IRI_HTTPS: &str =
	"https://schema.org/MerchantReturnEnumeration";
/// <https://schema.org/MerchantReturnEnumeration>
pub const MERCHANT_RETURN_ENUMERATION_LABEL: &str = "MerchantReturnEnumeration";
pub struct MerchantReturnEnumerationIri;
impl PartialEq<&str> for MerchantReturnEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MERCHANT_RETURN_ENUMERATION_IRI_HTTP
			|| *other == MERCHANT_RETURN_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<MerchantReturnEnumerationIri> for &str {
	fn eq(&self, other: &MerchantReturnEnumerationIri) -> bool {
		*self == MERCHANT_RETURN_ENUMERATION_IRI_HTTP
			|| *self == MERCHANT_RETURN_ENUMERATION_IRI_HTTPS
	}
}
pub struct MerchantReturnEnumerationIriOrLabel;
impl PartialEq<&str> for MerchantReturnEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MerchantReturnEnumerationIri || *other == MERCHANT_RETURN_ENUMERATION_LABEL
	}
}
impl PartialEq<MerchantReturnEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &MerchantReturnEnumerationIriOrLabel) -> bool {
		*self == MerchantReturnEnumerationIri || *self == MERCHANT_RETURN_ENUMERATION_LABEL
	}
}
