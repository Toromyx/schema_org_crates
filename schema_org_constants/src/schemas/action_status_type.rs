/// <https://schema.org/ActionStatusType>
pub const ACTION_STATUS_TYPE_IRI_HTTP: &str = "http://schema.org/ActionStatusType";
/// <https://schema.org/ActionStatusType>
pub const ACTION_STATUS_TYPE_IRI_HTTPS: &str = "https://schema.org/ActionStatusType";
/// <https://schema.org/ActionStatusType>
pub const ACTION_STATUS_TYPE_LABEL: &str = "ActionStatusType";
pub struct ActionStatusTypeIri;
impl PartialEq<&str> for ActionStatusTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACTION_STATUS_TYPE_IRI_HTTP || *other == ACTION_STATUS_TYPE_IRI_HTTPS
	}
}
impl PartialEq<ActionStatusTypeIri> for &str {
	fn eq(&self, other: &ActionStatusTypeIri) -> bool {
		*self == ACTION_STATUS_TYPE_IRI_HTTP || *self == ACTION_STATUS_TYPE_IRI_HTTPS
	}
}
pub struct ActionStatusTypeIriOrLabel;
impl PartialEq<&str> for ActionStatusTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ActionStatusTypeIri || *other == ACTION_STATUS_TYPE_LABEL
	}
}
impl PartialEq<ActionStatusTypeIriOrLabel> for &str {
	fn eq(&self, other: &ActionStatusTypeIriOrLabel) -> bool {
		*self == ActionStatusTypeIri || *self == ACTION_STATUS_TYPE_LABEL
	}
}
