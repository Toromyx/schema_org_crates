/// <https://schema.org/bccRecipient>
pub const BCC_RECIPIENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/bccRecipient";
/// <https://schema.org/bccRecipient>
pub const BCC_RECIPIENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/bccRecipient";
/// <https://schema.org/bccRecipient>
pub const BCC_RECIPIENT_PROPERTY_LABEL: &str = "bccRecipient";
pub struct BccRecipientPropertyIri;
impl PartialEq<&str> for BccRecipientPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BCC_RECIPIENT_PROPERTY_IRI_HTTP || *other == BCC_RECIPIENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BccRecipientPropertyIri> for &str {
	fn eq(&self, other: &BccRecipientPropertyIri) -> bool {
		*self == BCC_RECIPIENT_PROPERTY_IRI_HTTP || *self == BCC_RECIPIENT_PROPERTY_IRI_HTTPS
	}
}
pub struct BccRecipientPropertyIriOrLabel;
impl PartialEq<&str> for BccRecipientPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BccRecipientPropertyIri || *other == BCC_RECIPIENT_PROPERTY_LABEL
	}
}
impl PartialEq<BccRecipientPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BccRecipientPropertyIriOrLabel) -> bool {
		*self == BccRecipientPropertyIri || *self == BCC_RECIPIENT_PROPERTY_LABEL
	}
}
