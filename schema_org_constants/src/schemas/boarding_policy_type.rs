/// <https://schema.org/BoardingPolicyType>
pub const BOARDING_POLICY_TYPE_IRI_HTTP: &str = "http://schema.org/BoardingPolicyType";
/// <https://schema.org/BoardingPolicyType>
pub const BOARDING_POLICY_TYPE_IRI_HTTPS: &str = "https://schema.org/BoardingPolicyType";
/// <https://schema.org/BoardingPolicyType>
pub const BOARDING_POLICY_TYPE_LABEL: &str = "BoardingPolicyType";
pub struct BoardingPolicyTypeIri;
impl PartialEq<&str> for BoardingPolicyTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BOARDING_POLICY_TYPE_IRI_HTTP || *other == BOARDING_POLICY_TYPE_IRI_HTTPS
	}
}
impl PartialEq<BoardingPolicyTypeIri> for &str {
	fn eq(&self, other: &BoardingPolicyTypeIri) -> bool {
		*self == BOARDING_POLICY_TYPE_IRI_HTTP || *self == BOARDING_POLICY_TYPE_IRI_HTTPS
	}
}
pub struct BoardingPolicyTypeIriOrLabel;
impl PartialEq<&str> for BoardingPolicyTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BoardingPolicyTypeIri || *other == BOARDING_POLICY_TYPE_LABEL
	}
}
impl PartialEq<BoardingPolicyTypeIriOrLabel> for &str {
	fn eq(&self, other: &BoardingPolicyTypeIriOrLabel) -> bool {
		*self == BoardingPolicyTypeIri || *self == BOARDING_POLICY_TYPE_LABEL
	}
}
