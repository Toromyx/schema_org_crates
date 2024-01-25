/// <https://schema.org/membershipNumber>
pub const MEMBERSHIP_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/membershipNumber";
/// <https://schema.org/membershipNumber>
pub const MEMBERSHIP_NUMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/membershipNumber";
/// <https://schema.org/membershipNumber>
pub const MEMBERSHIP_NUMBER_PROPERTY_LABEL: &str = "membershipNumber";
pub struct MembershipNumberPropertyIri;
impl PartialEq<&str> for MembershipNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEMBERSHIP_NUMBER_PROPERTY_IRI_HTTP
			|| *other == MEMBERSHIP_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MembershipNumberPropertyIri> for &str {
	fn eq(&self, other: &MembershipNumberPropertyIri) -> bool {
		*self == MEMBERSHIP_NUMBER_PROPERTY_IRI_HTTP
			|| *self == MEMBERSHIP_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct MembershipNumberPropertyIriOrLabel;
impl PartialEq<&str> for MembershipNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MembershipNumberPropertyIri || *other == MEMBERSHIP_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<MembershipNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MembershipNumberPropertyIriOrLabel) -> bool {
		*self == MembershipNumberPropertyIri || *self == MEMBERSHIP_NUMBER_PROPERTY_LABEL
	}
}
