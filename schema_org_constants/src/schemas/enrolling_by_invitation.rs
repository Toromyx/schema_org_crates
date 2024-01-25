/// <https://schema.org/EnrollingByInvitation>
pub const ENROLLING_BY_INVITATION_IRI_HTTP: &str = "http://schema.org/EnrollingByInvitation";
/// <https://schema.org/EnrollingByInvitation>
pub const ENROLLING_BY_INVITATION_IRI_HTTPS: &str = "https://schema.org/EnrollingByInvitation";
/// <https://schema.org/EnrollingByInvitation>
pub const ENROLLING_BY_INVITATION_LABEL: &str = "EnrollingByInvitation";
pub struct EnrollingByInvitationIri;
impl PartialEq<&str> for EnrollingByInvitationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENROLLING_BY_INVITATION_IRI_HTTP || *other == ENROLLING_BY_INVITATION_IRI_HTTPS
	}
}
impl PartialEq<EnrollingByInvitationIri> for &str {
	fn eq(&self, other: &EnrollingByInvitationIri) -> bool {
		*self == ENROLLING_BY_INVITATION_IRI_HTTP || *self == ENROLLING_BY_INVITATION_IRI_HTTPS
	}
}
pub struct EnrollingByInvitationIriOrLabel;
impl PartialEq<&str> for EnrollingByInvitationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EnrollingByInvitationIri || *other == ENROLLING_BY_INVITATION_LABEL
	}
}
impl PartialEq<EnrollingByInvitationIriOrLabel> for &str {
	fn eq(&self, other: &EnrollingByInvitationIriOrLabel) -> bool {
		*self == EnrollingByInvitationIri || *self == ENROLLING_BY_INVITATION_LABEL
	}
}
