/// <https://schema.org/LeaveAction>
pub const LEAVE_ACTION_IRI_HTTP: &str = "http://schema.org/LeaveAction";
/// <https://schema.org/LeaveAction>
pub const LEAVE_ACTION_IRI_HTTPS: &str = "https://schema.org/LeaveAction";
/// <https://schema.org/LeaveAction>
pub const LEAVE_ACTION_LABEL: &str = "LeaveAction";
pub struct LeaveActionIri;
impl PartialEq<&str> for LeaveActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEAVE_ACTION_IRI_HTTP || *other == LEAVE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<LeaveActionIri> for &str {
	fn eq(&self, other: &LeaveActionIri) -> bool {
		*self == LEAVE_ACTION_IRI_HTTP || *self == LEAVE_ACTION_IRI_HTTPS
	}
}
pub struct LeaveActionIriOrLabel;
impl PartialEq<&str> for LeaveActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LeaveActionIri || *other == LEAVE_ACTION_LABEL
	}
}
impl PartialEq<LeaveActionIriOrLabel> for &str {
	fn eq(&self, other: &LeaveActionIriOrLabel) -> bool {
		*self == LeaveActionIri || *self == LEAVE_ACTION_LABEL
	}
}
