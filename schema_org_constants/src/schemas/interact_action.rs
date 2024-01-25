/// <https://schema.org/InteractAction>
pub const INTERACT_ACTION_IRI_HTTP: &str = "http://schema.org/InteractAction";
/// <https://schema.org/InteractAction>
pub const INTERACT_ACTION_IRI_HTTPS: &str = "https://schema.org/InteractAction";
/// <https://schema.org/InteractAction>
pub const INTERACT_ACTION_LABEL: &str = "InteractAction";
pub struct InteractActionIri;
impl PartialEq<&str> for InteractActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INTERACT_ACTION_IRI_HTTP || *other == INTERACT_ACTION_IRI_HTTPS
	}
}
impl PartialEq<InteractActionIri> for &str {
	fn eq(&self, other: &InteractActionIri) -> bool {
		*self == INTERACT_ACTION_IRI_HTTP || *self == INTERACT_ACTION_IRI_HTTPS
	}
}
pub struct InteractActionIriOrLabel;
impl PartialEq<&str> for InteractActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InteractActionIri || *other == INTERACT_ACTION_LABEL
	}
}
impl PartialEq<InteractActionIriOrLabel> for &str {
	fn eq(&self, other: &InteractActionIriOrLabel) -> bool {
		*self == InteractActionIri || *self == INTERACT_ACTION_LABEL
	}
}
