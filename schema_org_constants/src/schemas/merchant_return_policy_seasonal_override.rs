/// <https://schema.org/MerchantReturnPolicySeasonalOverride>
pub const MERCHANT_RETURN_POLICY_SEASONAL_OVERRIDE_IRI_HTTP: &str =
	"http://schema.org/MerchantReturnPolicySeasonalOverride";
/// <https://schema.org/MerchantReturnPolicySeasonalOverride>
pub const MERCHANT_RETURN_POLICY_SEASONAL_OVERRIDE_IRI_HTTPS: &str =
	"https://schema.org/MerchantReturnPolicySeasonalOverride";
/// <https://schema.org/MerchantReturnPolicySeasonalOverride>
pub const MERCHANT_RETURN_POLICY_SEASONAL_OVERRIDE_LABEL: &str =
	"MerchantReturnPolicySeasonalOverride";
pub struct MerchantReturnPolicySeasonalOverrideIri;
impl PartialEq<&str> for MerchantReturnPolicySeasonalOverrideIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MERCHANT_RETURN_POLICY_SEASONAL_OVERRIDE_IRI_HTTP
			|| *other == MERCHANT_RETURN_POLICY_SEASONAL_OVERRIDE_IRI_HTTPS
	}
}
impl PartialEq<MerchantReturnPolicySeasonalOverrideIri> for &str {
	fn eq(&self, other: &MerchantReturnPolicySeasonalOverrideIri) -> bool {
		*self == MERCHANT_RETURN_POLICY_SEASONAL_OVERRIDE_IRI_HTTP
			|| *self == MERCHANT_RETURN_POLICY_SEASONAL_OVERRIDE_IRI_HTTPS
	}
}
pub struct MerchantReturnPolicySeasonalOverrideIriOrLabel;
impl PartialEq<&str> for MerchantReturnPolicySeasonalOverrideIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MerchantReturnPolicySeasonalOverrideIri
			|| *other == MERCHANT_RETURN_POLICY_SEASONAL_OVERRIDE_LABEL
	}
}
impl PartialEq<MerchantReturnPolicySeasonalOverrideIriOrLabel> for &str {
	fn eq(&self, other: &MerchantReturnPolicySeasonalOverrideIriOrLabel) -> bool {
		*self == MerchantReturnPolicySeasonalOverrideIri
			|| *self == MERCHANT_RETURN_POLICY_SEASONAL_OVERRIDE_LABEL
	}
}
