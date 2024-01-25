/// <https://schema.org/programMembershipUsed>
pub const PROGRAM_MEMBERSHIP_USED_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/programMembershipUsed";
/// <https://schema.org/programMembershipUsed>
pub const PROGRAM_MEMBERSHIP_USED_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/programMembershipUsed";
/// <https://schema.org/programMembershipUsed>
pub const PROGRAM_MEMBERSHIP_USED_PROPERTY_LABEL: &str = "programMembershipUsed";
pub struct ProgramMembershipUsedPropertyIri;
impl PartialEq<&str> for ProgramMembershipUsedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROGRAM_MEMBERSHIP_USED_PROPERTY_IRI_HTTP
			|| *other == PROGRAM_MEMBERSHIP_USED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProgramMembershipUsedPropertyIri> for &str {
	fn eq(&self, other: &ProgramMembershipUsedPropertyIri) -> bool {
		*self == PROGRAM_MEMBERSHIP_USED_PROPERTY_IRI_HTTP
			|| *self == PROGRAM_MEMBERSHIP_USED_PROPERTY_IRI_HTTPS
	}
}
pub struct ProgramMembershipUsedPropertyIriOrLabel;
impl PartialEq<&str> for ProgramMembershipUsedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProgramMembershipUsedPropertyIri
			|| *other == PROGRAM_MEMBERSHIP_USED_PROPERTY_LABEL
	}
}
impl PartialEq<ProgramMembershipUsedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProgramMembershipUsedPropertyIriOrLabel) -> bool {
		*self == ProgramMembershipUsedPropertyIri || *self == PROGRAM_MEMBERSHIP_USED_PROPERTY_LABEL
	}
}
