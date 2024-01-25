/// <https://schema.org/ReceiveAction>
pub const RECEIVE_ACTION_IRI_HTTP: &str = "http://schema.org/ReceiveAction";
/// <https://schema.org/ReceiveAction>
pub const RECEIVE_ACTION_IRI_HTTPS: &str = "https://schema.org/ReceiveAction";
/// <https://schema.org/ReceiveAction>
pub const RECEIVE_ACTION_LABEL: &str = "ReceiveAction";
pub struct ReceiveActionIri;
impl PartialEq<&str> for ReceiveActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECEIVE_ACTION_IRI_HTTP || *other == RECEIVE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<ReceiveActionIri> for &str {
	fn eq(&self, other: &ReceiveActionIri) -> bool {
		*self == RECEIVE_ACTION_IRI_HTTP || *self == RECEIVE_ACTION_IRI_HTTPS
	}
}
pub struct ReceiveActionIriOrLabel;
impl PartialEq<&str> for ReceiveActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReceiveActionIri || *other == RECEIVE_ACTION_LABEL
	}
}
impl PartialEq<ReceiveActionIriOrLabel> for &str {
	fn eq(&self, other: &ReceiveActionIriOrLabel) -> bool {
		*self == ReceiveActionIri || *self == RECEIVE_ACTION_LABEL
	}
}
