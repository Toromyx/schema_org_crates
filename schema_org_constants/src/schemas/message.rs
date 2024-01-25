/// <https://schema.org/Message>
pub const MESSAGE_IRI_HTTP: &str = "http://schema.org/Message";
/// <https://schema.org/Message>
pub const MESSAGE_IRI_HTTPS: &str = "https://schema.org/Message";
/// <https://schema.org/Message>
pub const MESSAGE_LABEL: &str = "Message";
pub struct MessageIri;
impl PartialEq<&str> for MessageIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MESSAGE_IRI_HTTP || *other == MESSAGE_IRI_HTTPS
	}
}
impl PartialEq<MessageIri> for &str {
	fn eq(&self, other: &MessageIri) -> bool {
		*self == MESSAGE_IRI_HTTP || *self == MESSAGE_IRI_HTTPS
	}
}
pub struct MessageIriOrLabel;
impl PartialEq<&str> for MessageIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MessageIri || *other == MESSAGE_LABEL
	}
}
impl PartialEq<MessageIriOrLabel> for &str {
	fn eq(&self, other: &MessageIriOrLabel) -> bool {
		*self == MessageIri || *self == MESSAGE_LABEL
	}
}
