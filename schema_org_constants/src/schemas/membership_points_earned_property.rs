/// <https://schema.org/membershipPointsEarned>
pub const MEMBERSHIP_POINTS_EARNED_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/membershipPointsEarned";
/// <https://schema.org/membershipPointsEarned>
pub const MEMBERSHIP_POINTS_EARNED_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/membershipPointsEarned";
/// <https://schema.org/membershipPointsEarned>
pub const MEMBERSHIP_POINTS_EARNED_PROPERTY_LABEL: &str = "membershipPointsEarned";
pub struct MembershipPointsEarnedPropertyIri;
impl PartialEq<&str> for MembershipPointsEarnedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEMBERSHIP_POINTS_EARNED_PROPERTY_IRI_HTTP
			|| *other == MEMBERSHIP_POINTS_EARNED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MembershipPointsEarnedPropertyIri> for &str {
	fn eq(&self, other: &MembershipPointsEarnedPropertyIri) -> bool {
		*self == MEMBERSHIP_POINTS_EARNED_PROPERTY_IRI_HTTP
			|| *self == MEMBERSHIP_POINTS_EARNED_PROPERTY_IRI_HTTPS
	}
}
pub struct MembershipPointsEarnedPropertyIriOrLabel;
impl PartialEq<&str> for MembershipPointsEarnedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MembershipPointsEarnedPropertyIri
			|| *other == MEMBERSHIP_POINTS_EARNED_PROPERTY_LABEL
	}
}
impl PartialEq<MembershipPointsEarnedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MembershipPointsEarnedPropertyIriOrLabel) -> bool {
		*self == MembershipPointsEarnedPropertyIri
			|| *self == MEMBERSHIP_POINTS_EARNED_PROPERTY_LABEL
	}
}
