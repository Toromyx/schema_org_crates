/// <https://schema.org/DeactivateAction>
pub const DEACTIVATE_ACTION_IRI_HTTP: &str = "http://schema.org/DeactivateAction";
/// <https://schema.org/DeactivateAction>
pub const DEACTIVATE_ACTION_IRI_HTTPS: &str = "https://schema.org/DeactivateAction";
/// <https://schema.org/DeactivateAction>
pub const DEACTIVATE_ACTION_LABEL: &str = "DeactivateAction";
pub struct DeactivateActionIri;
impl PartialEq<&str> for DeactivateActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEACTIVATE_ACTION_IRI_HTTP || *other == DEACTIVATE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<DeactivateActionIri> for &str {
	fn eq(&self, other: &DeactivateActionIri) -> bool {
		*self == DEACTIVATE_ACTION_IRI_HTTP || *self == DEACTIVATE_ACTION_IRI_HTTPS
	}
}
pub struct DeactivateActionIriOrLabel;
impl PartialEq<&str> for DeactivateActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DeactivateActionIri || *other == DEACTIVATE_ACTION_LABEL
	}
}
impl PartialEq<DeactivateActionIriOrLabel> for &str {
	fn eq(&self, other: &DeactivateActionIriOrLabel) -> bool {
		*self == DeactivateActionIri || *self == DEACTIVATE_ACTION_LABEL
	}
}
