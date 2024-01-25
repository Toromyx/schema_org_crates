/// <https://schema.org/EmailMessage>
pub const EMAIL_MESSAGE_IRI_HTTP: &str = "http://schema.org/EmailMessage";
/// <https://schema.org/EmailMessage>
pub const EMAIL_MESSAGE_IRI_HTTPS: &str = "https://schema.org/EmailMessage";
/// <https://schema.org/EmailMessage>
pub const EMAIL_MESSAGE_LABEL: &str = "EmailMessage";
pub struct EmailMessageIri;
impl PartialEq<&str> for EmailMessageIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EMAIL_MESSAGE_IRI_HTTP || *other == EMAIL_MESSAGE_IRI_HTTPS
	}
}
impl PartialEq<EmailMessageIri> for &str {
	fn eq(&self, other: &EmailMessageIri) -> bool {
		*self == EMAIL_MESSAGE_IRI_HTTP || *self == EMAIL_MESSAGE_IRI_HTTPS
	}
}
pub struct EmailMessageIriOrLabel;
impl PartialEq<&str> for EmailMessageIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EmailMessageIri || *other == EMAIL_MESSAGE_LABEL
	}
}
impl PartialEq<EmailMessageIriOrLabel> for &str {
	fn eq(&self, other: &EmailMessageIriOrLabel) -> bool {
		*self == EmailMessageIri || *self == EMAIL_MESSAGE_LABEL
	}
}
