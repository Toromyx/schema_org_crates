/// <https://schema.org/MerchantReturnUnspecified>
pub const MERCHANT_RETURN_UNSPECIFIED_IRI_HTTP: &str =
	"http://schema.org/MerchantReturnUnspecified";
/// <https://schema.org/MerchantReturnUnspecified>
pub const MERCHANT_RETURN_UNSPECIFIED_IRI_HTTPS: &str =
	"https://schema.org/MerchantReturnUnspecified";
/// <https://schema.org/MerchantReturnUnspecified>
pub const MERCHANT_RETURN_UNSPECIFIED_LABEL: &str = "MerchantReturnUnspecified";
pub struct MerchantReturnUnspecifiedIri;
impl PartialEq<&str> for MerchantReturnUnspecifiedIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MERCHANT_RETURN_UNSPECIFIED_IRI_HTTP
			|| *other == MERCHANT_RETURN_UNSPECIFIED_IRI_HTTPS
	}
}
impl PartialEq<MerchantReturnUnspecifiedIri> for &str {
	fn eq(&self, other: &MerchantReturnUnspecifiedIri) -> bool {
		*self == MERCHANT_RETURN_UNSPECIFIED_IRI_HTTP
			|| *self == MERCHANT_RETURN_UNSPECIFIED_IRI_HTTPS
	}
}
pub struct MerchantReturnUnspecifiedIriOrLabel;
impl PartialEq<&str> for MerchantReturnUnspecifiedIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MerchantReturnUnspecifiedIri || *other == MERCHANT_RETURN_UNSPECIFIED_LABEL
	}
}
impl PartialEq<MerchantReturnUnspecifiedIriOrLabel> for &str {
	fn eq(&self, other: &MerchantReturnUnspecifiedIriOrLabel) -> bool {
		*self == MerchantReturnUnspecifiedIri || *self == MERCHANT_RETURN_UNSPECIFIED_LABEL
	}
}
