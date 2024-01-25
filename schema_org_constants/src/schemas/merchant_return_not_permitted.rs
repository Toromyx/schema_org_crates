/// <https://schema.org/MerchantReturnNotPermitted>
pub const MERCHANT_RETURN_NOT_PERMITTED_IRI_HTTP: &str =
	"http://schema.org/MerchantReturnNotPermitted";
/// <https://schema.org/MerchantReturnNotPermitted>
pub const MERCHANT_RETURN_NOT_PERMITTED_IRI_HTTPS: &str =
	"https://schema.org/MerchantReturnNotPermitted";
/// <https://schema.org/MerchantReturnNotPermitted>
pub const MERCHANT_RETURN_NOT_PERMITTED_LABEL: &str = "MerchantReturnNotPermitted";
pub struct MerchantReturnNotPermittedIri;
impl PartialEq<&str> for MerchantReturnNotPermittedIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MERCHANT_RETURN_NOT_PERMITTED_IRI_HTTP
			|| *other == MERCHANT_RETURN_NOT_PERMITTED_IRI_HTTPS
	}
}
impl PartialEq<MerchantReturnNotPermittedIri> for &str {
	fn eq(&self, other: &MerchantReturnNotPermittedIri) -> bool {
		*self == MERCHANT_RETURN_NOT_PERMITTED_IRI_HTTP
			|| *self == MERCHANT_RETURN_NOT_PERMITTED_IRI_HTTPS
	}
}
pub struct MerchantReturnNotPermittedIriOrLabel;
impl PartialEq<&str> for MerchantReturnNotPermittedIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MerchantReturnNotPermittedIri || *other == MERCHANT_RETURN_NOT_PERMITTED_LABEL
	}
}
impl PartialEq<MerchantReturnNotPermittedIriOrLabel> for &str {
	fn eq(&self, other: &MerchantReturnNotPermittedIriOrLabel) -> bool {
		*self == MerchantReturnNotPermittedIri || *self == MERCHANT_RETURN_NOT_PERMITTED_LABEL
	}
}
