/// <https://schema.org/messageAttachment>
pub const MESSAGE_ATTACHMENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/messageAttachment";
/// <https://schema.org/messageAttachment>
pub const MESSAGE_ATTACHMENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/messageAttachment";
/// <https://schema.org/messageAttachment>
pub const MESSAGE_ATTACHMENT_PROPERTY_LABEL: &str = "messageAttachment";
pub struct MessageAttachmentPropertyIri;
impl PartialEq<&str> for MessageAttachmentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MESSAGE_ATTACHMENT_PROPERTY_IRI_HTTP
			|| *other == MESSAGE_ATTACHMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MessageAttachmentPropertyIri> for &str {
	fn eq(&self, other: &MessageAttachmentPropertyIri) -> bool {
		*self == MESSAGE_ATTACHMENT_PROPERTY_IRI_HTTP
			|| *self == MESSAGE_ATTACHMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct MessageAttachmentPropertyIriOrLabel;
impl PartialEq<&str> for MessageAttachmentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MessageAttachmentPropertyIri || *other == MESSAGE_ATTACHMENT_PROPERTY_LABEL
	}
}
impl PartialEq<MessageAttachmentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MessageAttachmentPropertyIriOrLabel) -> bool {
		*self == MessageAttachmentPropertyIri || *self == MESSAGE_ATTACHMENT_PROPERTY_LABEL
	}
}
