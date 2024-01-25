/// <https://schema.org/ActivateAction>
pub const ACTIVATE_ACTION_IRI_HTTP: &str = "http://schema.org/ActivateAction";
/// <https://schema.org/ActivateAction>
pub const ACTIVATE_ACTION_IRI_HTTPS: &str = "https://schema.org/ActivateAction";
/// <https://schema.org/ActivateAction>
pub const ACTIVATE_ACTION_LABEL: &str = "ActivateAction";
pub struct ActivateActionIri;
impl PartialEq<&str> for ActivateActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACTIVATE_ACTION_IRI_HTTP || *other == ACTIVATE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<ActivateActionIri> for &str {
	fn eq(&self, other: &ActivateActionIri) -> bool {
		*self == ACTIVATE_ACTION_IRI_HTTP || *self == ACTIVATE_ACTION_IRI_HTTPS
	}
}
pub struct ActivateActionIriOrLabel;
impl PartialEq<&str> for ActivateActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ActivateActionIri || *other == ACTIVATE_ACTION_LABEL
	}
}
impl PartialEq<ActivateActionIriOrLabel> for &str {
	fn eq(&self, other: &ActivateActionIriOrLabel) -> bool {
		*self == ActivateActionIri || *self == ACTIVATE_ACTION_LABEL
	}
}
