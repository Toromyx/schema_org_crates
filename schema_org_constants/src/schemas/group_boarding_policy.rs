/// <https://schema.org/GroupBoardingPolicy>
pub const GROUP_BOARDING_POLICY_IRI_HTTP: &str = "http://schema.org/GroupBoardingPolicy";
/// <https://schema.org/GroupBoardingPolicy>
pub const GROUP_BOARDING_POLICY_IRI_HTTPS: &str = "https://schema.org/GroupBoardingPolicy";
/// <https://schema.org/GroupBoardingPolicy>
pub const GROUP_BOARDING_POLICY_LABEL: &str = "GroupBoardingPolicy";
pub struct GroupBoardingPolicyIri;
impl PartialEq<&str> for GroupBoardingPolicyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GROUP_BOARDING_POLICY_IRI_HTTP || *other == GROUP_BOARDING_POLICY_IRI_HTTPS
	}
}
impl PartialEq<GroupBoardingPolicyIri> for &str {
	fn eq(&self, other: &GroupBoardingPolicyIri) -> bool {
		*self == GROUP_BOARDING_POLICY_IRI_HTTP || *self == GROUP_BOARDING_POLICY_IRI_HTTPS
	}
}
pub struct GroupBoardingPolicyIriOrLabel;
impl PartialEq<&str> for GroupBoardingPolicyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GroupBoardingPolicyIri || *other == GROUP_BOARDING_POLICY_LABEL
	}
}
impl PartialEq<GroupBoardingPolicyIriOrLabel> for &str {
	fn eq(&self, other: &GroupBoardingPolicyIriOrLabel) -> bool {
		*self == GroupBoardingPolicyIri || *self == GROUP_BOARDING_POLICY_LABEL
	}
}
