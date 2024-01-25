/// <https://schema.org/hasMerchantReturnPolicy>
pub const HAS_MERCHANT_RETURN_POLICY_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/hasMerchantReturnPolicy";
/// <https://schema.org/hasMerchantReturnPolicy>
pub const HAS_MERCHANT_RETURN_POLICY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/hasMerchantReturnPolicy";
/// <https://schema.org/hasMerchantReturnPolicy>
pub const HAS_MERCHANT_RETURN_POLICY_PROPERTY_LABEL: &str = "hasMerchantReturnPolicy";
pub struct HasMerchantReturnPolicyPropertyIri;
impl PartialEq<&str> for HasMerchantReturnPolicyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_MERCHANT_RETURN_POLICY_PROPERTY_IRI_HTTP
			|| *other == HAS_MERCHANT_RETURN_POLICY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasMerchantReturnPolicyPropertyIri> for &str {
	fn eq(&self, other: &HasMerchantReturnPolicyPropertyIri) -> bool {
		*self == HAS_MERCHANT_RETURN_POLICY_PROPERTY_IRI_HTTP
			|| *self == HAS_MERCHANT_RETURN_POLICY_PROPERTY_IRI_HTTPS
	}
}
pub struct HasMerchantReturnPolicyPropertyIriOrLabel;
impl PartialEq<&str> for HasMerchantReturnPolicyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasMerchantReturnPolicyPropertyIri
			|| *other == HAS_MERCHANT_RETURN_POLICY_PROPERTY_LABEL
	}
}
impl PartialEq<HasMerchantReturnPolicyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasMerchantReturnPolicyPropertyIriOrLabel) -> bool {
		*self == HasMerchantReturnPolicyPropertyIri
			|| *self == HAS_MERCHANT_RETURN_POLICY_PROPERTY_LABEL
	}
}
