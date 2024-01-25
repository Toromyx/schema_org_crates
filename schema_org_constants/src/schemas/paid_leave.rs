/// <https://schema.org/PaidLeave>
pub const PAID_LEAVE_IRI_HTTP: &str = "http://schema.org/PaidLeave";
/// <https://schema.org/PaidLeave>
pub const PAID_LEAVE_IRI_HTTPS: &str = "https://schema.org/PaidLeave";
/// <https://schema.org/PaidLeave>
pub const PAID_LEAVE_LABEL: &str = "PaidLeave";
pub struct PaidLeaveIri;
impl PartialEq<&str> for PaidLeaveIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAID_LEAVE_IRI_HTTP || *other == PAID_LEAVE_IRI_HTTPS
	}
}
impl PartialEq<PaidLeaveIri> for &str {
	fn eq(&self, other: &PaidLeaveIri) -> bool {
		*self == PAID_LEAVE_IRI_HTTP || *self == PAID_LEAVE_IRI_HTTPS
	}
}
pub struct PaidLeaveIriOrLabel;
impl PartialEq<&str> for PaidLeaveIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PaidLeaveIri || *other == PAID_LEAVE_LABEL
	}
}
impl PartialEq<PaidLeaveIriOrLabel> for &str {
	fn eq(&self, other: &PaidLeaveIriOrLabel) -> bool {
		*self == PaidLeaveIri || *self == PAID_LEAVE_LABEL
	}
}
