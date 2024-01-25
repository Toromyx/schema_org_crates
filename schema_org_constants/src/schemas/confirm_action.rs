/// <https://schema.org/ConfirmAction>
pub const CONFIRM_ACTION_IRI_HTTP: &str = "http://schema.org/ConfirmAction";
/// <https://schema.org/ConfirmAction>
pub const CONFIRM_ACTION_IRI_HTTPS: &str = "https://schema.org/ConfirmAction";
/// <https://schema.org/ConfirmAction>
pub const CONFIRM_ACTION_LABEL: &str = "ConfirmAction";
pub struct ConfirmActionIri;
impl PartialEq<&str> for ConfirmActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONFIRM_ACTION_IRI_HTTP || *other == CONFIRM_ACTION_IRI_HTTPS
	}
}
impl PartialEq<ConfirmActionIri> for &str {
	fn eq(&self, other: &ConfirmActionIri) -> bool {
		*self == CONFIRM_ACTION_IRI_HTTP || *self == CONFIRM_ACTION_IRI_HTTPS
	}
}
pub struct ConfirmActionIriOrLabel;
impl PartialEq<&str> for ConfirmActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ConfirmActionIri || *other == CONFIRM_ACTION_LABEL
	}
}
impl PartialEq<ConfirmActionIriOrLabel> for &str {
	fn eq(&self, other: &ConfirmActionIriOrLabel) -> bool {
		*self == ConfirmActionIri || *self == CONFIRM_ACTION_LABEL
	}
}
