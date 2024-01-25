/// <https://schema.org/toRecipient>
pub const TO_RECIPIENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/toRecipient";
/// <https://schema.org/toRecipient>
pub const TO_RECIPIENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/toRecipient";
/// <https://schema.org/toRecipient>
pub const TO_RECIPIENT_PROPERTY_LABEL: &str = "toRecipient";
pub struct ToRecipientPropertyIri;
impl PartialEq<&str> for ToRecipientPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TO_RECIPIENT_PROPERTY_IRI_HTTP || *other == TO_RECIPIENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ToRecipientPropertyIri> for &str {
	fn eq(&self, other: &ToRecipientPropertyIri) -> bool {
		*self == TO_RECIPIENT_PROPERTY_IRI_HTTP || *self == TO_RECIPIENT_PROPERTY_IRI_HTTPS
	}
}
pub struct ToRecipientPropertyIriOrLabel;
impl PartialEq<&str> for ToRecipientPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ToRecipientPropertyIri || *other == TO_RECIPIENT_PROPERTY_LABEL
	}
}
impl PartialEq<ToRecipientPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ToRecipientPropertyIriOrLabel) -> bool {
		*self == ToRecipientPropertyIri || *self == TO_RECIPIENT_PROPERTY_LABEL
	}
}
