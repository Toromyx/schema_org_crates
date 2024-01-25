/// <https://schema.org/boardingPolicy>
pub const BOARDING_POLICY_PROPERTY_IRI_HTTP: &str = "http://schema.org/boardingPolicy";
/// <https://schema.org/boardingPolicy>
pub const BOARDING_POLICY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/boardingPolicy";
/// <https://schema.org/boardingPolicy>
pub const BOARDING_POLICY_PROPERTY_LABEL: &str = "boardingPolicy";
pub struct BoardingPolicyPropertyIri;
impl PartialEq<&str> for BoardingPolicyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BOARDING_POLICY_PROPERTY_IRI_HTTP || *other == BOARDING_POLICY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BoardingPolicyPropertyIri> for &str {
	fn eq(&self, other: &BoardingPolicyPropertyIri) -> bool {
		*self == BOARDING_POLICY_PROPERTY_IRI_HTTP || *self == BOARDING_POLICY_PROPERTY_IRI_HTTPS
	}
}
pub struct BoardingPolicyPropertyIriOrLabel;
impl PartialEq<&str> for BoardingPolicyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BoardingPolicyPropertyIri || *other == BOARDING_POLICY_PROPERTY_LABEL
	}
}
impl PartialEq<BoardingPolicyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BoardingPolicyPropertyIriOrLabel) -> bool {
		*self == BoardingPolicyPropertyIri || *self == BOARDING_POLICY_PROPERTY_LABEL
	}
}
