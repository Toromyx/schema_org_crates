/// <https://schema.org/ZoneBoardingPolicy>
pub const ZONE_BOARDING_POLICY_IRI_HTTP: &str = "http://schema.org/ZoneBoardingPolicy";
/// <https://schema.org/ZoneBoardingPolicy>
pub const ZONE_BOARDING_POLICY_IRI_HTTPS: &str = "https://schema.org/ZoneBoardingPolicy";
/// <https://schema.org/ZoneBoardingPolicy>
pub const ZONE_BOARDING_POLICY_LABEL: &str = "ZoneBoardingPolicy";
pub struct ZoneBoardingPolicyIri;
impl PartialEq<&str> for ZoneBoardingPolicyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ZONE_BOARDING_POLICY_IRI_HTTP || *other == ZONE_BOARDING_POLICY_IRI_HTTPS
	}
}
impl PartialEq<ZoneBoardingPolicyIri> for &str {
	fn eq(&self, other: &ZoneBoardingPolicyIri) -> bool {
		*self == ZONE_BOARDING_POLICY_IRI_HTTP || *self == ZONE_BOARDING_POLICY_IRI_HTTPS
	}
}
pub struct ZoneBoardingPolicyIriOrLabel;
impl PartialEq<&str> for ZoneBoardingPolicyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ZoneBoardingPolicyIri || *other == ZONE_BOARDING_POLICY_LABEL
	}
}
impl PartialEq<ZoneBoardingPolicyIriOrLabel> for &str {
	fn eq(&self, other: &ZoneBoardingPolicyIriOrLabel) -> bool {
		*self == ZoneBoardingPolicyIri || *self == ZONE_BOARDING_POLICY_LABEL
	}
}
