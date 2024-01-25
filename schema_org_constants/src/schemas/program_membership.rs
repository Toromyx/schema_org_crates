/// <https://schema.org/ProgramMembership>
pub const PROGRAM_MEMBERSHIP_IRI_HTTP: &str = "http://schema.org/ProgramMembership";
/// <https://schema.org/ProgramMembership>
pub const PROGRAM_MEMBERSHIP_IRI_HTTPS: &str = "https://schema.org/ProgramMembership";
/// <https://schema.org/ProgramMembership>
pub const PROGRAM_MEMBERSHIP_LABEL: &str = "ProgramMembership";
pub struct ProgramMembershipIri;
impl PartialEq<&str> for ProgramMembershipIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROGRAM_MEMBERSHIP_IRI_HTTP || *other == PROGRAM_MEMBERSHIP_IRI_HTTPS
	}
}
impl PartialEq<ProgramMembershipIri> for &str {
	fn eq(&self, other: &ProgramMembershipIri) -> bool {
		*self == PROGRAM_MEMBERSHIP_IRI_HTTP || *self == PROGRAM_MEMBERSHIP_IRI_HTTPS
	}
}
pub struct ProgramMembershipIriOrLabel;
impl PartialEq<&str> for ProgramMembershipIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProgramMembershipIri || *other == PROGRAM_MEMBERSHIP_LABEL
	}
}
impl PartialEq<ProgramMembershipIriOrLabel> for &str {
	fn eq(&self, other: &ProgramMembershipIriOrLabel) -> bool {
		*self == ProgramMembershipIri || *self == PROGRAM_MEMBERSHIP_LABEL
	}
}
