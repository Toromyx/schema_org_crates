/// <https://schema.org/MerchantReturnPolicy>
pub const MERCHANT_RETURN_POLICY_IRI_HTTP: &str = "http://schema.org/MerchantReturnPolicy";
/// <https://schema.org/MerchantReturnPolicy>
pub const MERCHANT_RETURN_POLICY_IRI_HTTPS: &str = "https://schema.org/MerchantReturnPolicy";
/// <https://schema.org/MerchantReturnPolicy>
pub const MERCHANT_RETURN_POLICY_LABEL: &str = "MerchantReturnPolicy";
pub struct MerchantReturnPolicyIri;
impl PartialEq<&str> for MerchantReturnPolicyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MERCHANT_RETURN_POLICY_IRI_HTTP || *other == MERCHANT_RETURN_POLICY_IRI_HTTPS
	}
}
impl PartialEq<MerchantReturnPolicyIri> for &str {
	fn eq(&self, other: &MerchantReturnPolicyIri) -> bool {
		*self == MERCHANT_RETURN_POLICY_IRI_HTTP || *self == MERCHANT_RETURN_POLICY_IRI_HTTPS
	}
}
pub struct MerchantReturnPolicyIriOrLabel;
impl PartialEq<&str> for MerchantReturnPolicyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MerchantReturnPolicyIri || *other == MERCHANT_RETURN_POLICY_LABEL
	}
}
impl PartialEq<MerchantReturnPolicyIriOrLabel> for &str {
	fn eq(&self, other: &MerchantReturnPolicyIriOrLabel) -> bool {
		*self == MerchantReturnPolicyIri || *self == MERCHANT_RETURN_POLICY_LABEL
	}
}
