/// <https://schema.org/SendAction>
pub const SEND_ACTION_IRI_HTTP: &str = "http://schema.org/SendAction";
/// <https://schema.org/SendAction>
pub const SEND_ACTION_IRI_HTTPS: &str = "https://schema.org/SendAction";
/// <https://schema.org/SendAction>
pub const SEND_ACTION_LABEL: &str = "SendAction";
pub struct SendActionIri;
impl PartialEq<&str> for SendActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SEND_ACTION_IRI_HTTP || *other == SEND_ACTION_IRI_HTTPS
	}
}
impl PartialEq<SendActionIri> for &str {
	fn eq(&self, other: &SendActionIri) -> bool {
		*self == SEND_ACTION_IRI_HTTP || *self == SEND_ACTION_IRI_HTTPS
	}
}
pub struct SendActionIriOrLabel;
impl PartialEq<&str> for SendActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SendActionIri || *other == SEND_ACTION_LABEL
	}
}
impl PartialEq<SendActionIriOrLabel> for &str {
	fn eq(&self, other: &SendActionIriOrLabel) -> bool {
		*self == SendActionIri || *self == SEND_ACTION_LABEL
	}
}
