/// <https://schema.org/returnPolicySeasonalOverride>
pub const RETURN_POLICY_SEASONAL_OVERRIDE_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/returnPolicySeasonalOverride";
/// <https://schema.org/returnPolicySeasonalOverride>
pub const RETURN_POLICY_SEASONAL_OVERRIDE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/returnPolicySeasonalOverride";
/// <https://schema.org/returnPolicySeasonalOverride>
pub const RETURN_POLICY_SEASONAL_OVERRIDE_PROPERTY_LABEL: &str = "returnPolicySeasonalOverride";
pub struct ReturnPolicySeasonalOverridePropertyIri;
impl PartialEq<&str> for ReturnPolicySeasonalOverridePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RETURN_POLICY_SEASONAL_OVERRIDE_PROPERTY_IRI_HTTP
			|| *other == RETURN_POLICY_SEASONAL_OVERRIDE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReturnPolicySeasonalOverridePropertyIri> for &str {
	fn eq(&self, other: &ReturnPolicySeasonalOverridePropertyIri) -> bool {
		*self == RETURN_POLICY_SEASONAL_OVERRIDE_PROPERTY_IRI_HTTP
			|| *self == RETURN_POLICY_SEASONAL_OVERRIDE_PROPERTY_IRI_HTTPS
	}
}
pub struct ReturnPolicySeasonalOverridePropertyIriOrLabel;
impl PartialEq<&str> for ReturnPolicySeasonalOverridePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReturnPolicySeasonalOverridePropertyIri
			|| *other == RETURN_POLICY_SEASONAL_OVERRIDE_PROPERTY_LABEL
	}
}
impl PartialEq<ReturnPolicySeasonalOverridePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReturnPolicySeasonalOverridePropertyIriOrLabel) -> bool {
		*self == ReturnPolicySeasonalOverridePropertyIri
			|| *self == RETURN_POLICY_SEASONAL_OVERRIDE_PROPERTY_LABEL
	}
}
