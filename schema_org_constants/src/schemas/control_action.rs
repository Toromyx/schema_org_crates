/// <https://schema.org/ControlAction>
pub const CONTROL_ACTION_IRI_HTTP: &str = "http://schema.org/ControlAction";
/// <https://schema.org/ControlAction>
pub const CONTROL_ACTION_IRI_HTTPS: &str = "https://schema.org/ControlAction";
/// <https://schema.org/ControlAction>
pub const CONTROL_ACTION_LABEL: &str = "ControlAction";
pub struct ControlActionIri;
impl PartialEq<&str> for ControlActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONTROL_ACTION_IRI_HTTP || *other == CONTROL_ACTION_IRI_HTTPS
	}
}
impl PartialEq<ControlActionIri> for &str {
	fn eq(&self, other: &ControlActionIri) -> bool {
		*self == CONTROL_ACTION_IRI_HTTP || *self == CONTROL_ACTION_IRI_HTTPS
	}
}
pub struct ControlActionIriOrLabel;
impl PartialEq<&str> for ControlActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ControlActionIri || *other == CONTROL_ACTION_LABEL
	}
}
impl PartialEq<ControlActionIriOrLabel> for &str {
	fn eq(&self, other: &ControlActionIriOrLabel) -> bool {
		*self == ControlActionIri || *self == CONTROL_ACTION_LABEL
	}
}
