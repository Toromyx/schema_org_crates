/// <https://schema.org/AssignAction>
pub const ASSIGN_ACTION_IRI_HTTP: &str = "http://schema.org/AssignAction";
/// <https://schema.org/AssignAction>
pub const ASSIGN_ACTION_IRI_HTTPS: &str = "https://schema.org/AssignAction";
/// <https://schema.org/AssignAction>
pub const ASSIGN_ACTION_LABEL: &str = "AssignAction";
pub struct AssignActionIri;
impl PartialEq<&str> for AssignActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ASSIGN_ACTION_IRI_HTTP || *other == ASSIGN_ACTION_IRI_HTTPS
	}
}
impl PartialEq<AssignActionIri> for &str {
	fn eq(&self, other: &AssignActionIri) -> bool {
		*self == ASSIGN_ACTION_IRI_HTTP || *self == ASSIGN_ACTION_IRI_HTTPS
	}
}
pub struct AssignActionIriOrLabel;
impl PartialEq<&str> for AssignActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AssignActionIri || *other == ASSIGN_ACTION_LABEL
	}
}
impl PartialEq<AssignActionIriOrLabel> for &str {
	fn eq(&self, other: &AssignActionIriOrLabel) -> bool {
		*self == AssignActionIri || *self == ASSIGN_ACTION_LABEL
	}
}
