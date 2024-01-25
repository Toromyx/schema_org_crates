/// <https://schema.org/CommunicateAction>
pub const COMMUNICATE_ACTION_IRI_HTTP: &str = "http://schema.org/CommunicateAction";
/// <https://schema.org/CommunicateAction>
pub const COMMUNICATE_ACTION_IRI_HTTPS: &str = "https://schema.org/CommunicateAction";
/// <https://schema.org/CommunicateAction>
pub const COMMUNICATE_ACTION_LABEL: &str = "CommunicateAction";
pub struct CommunicateActionIri;
impl PartialEq<&str> for CommunicateActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMMUNICATE_ACTION_IRI_HTTP || *other == COMMUNICATE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<CommunicateActionIri> for &str {
	fn eq(&self, other: &CommunicateActionIri) -> bool {
		*self == COMMUNICATE_ACTION_IRI_HTTP || *self == COMMUNICATE_ACTION_IRI_HTTPS
	}
}
pub struct CommunicateActionIriOrLabel;
impl PartialEq<&str> for CommunicateActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CommunicateActionIri || *other == COMMUNICATE_ACTION_LABEL
	}
}
impl PartialEq<CommunicateActionIriOrLabel> for &str {
	fn eq(&self, other: &CommunicateActionIriOrLabel) -> bool {
		*self == CommunicateActionIri || *self == COMMUNICATE_ACTION_LABEL
	}
}
